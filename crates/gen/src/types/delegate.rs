use super::*;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Delegate(pub TypeDef);

impl Delegate {
    pub fn gen(&self, gen: &Gen) -> TokenStream {
        let name = gen_type_name(&self.0, gen);
        let abi_name = gen_abi_name(&self.0, gen);
        let turbo_abi_name = gen_turbo_abi_name(&self.0, gen);
        let signature = self.0.invoke_method().signature(&self.0.generics);
        let abi_signature = gen_winrt_abi(&signature, gen);
        let fn_constraint = gen_winrt_constraint(&signature, gen);
        let guid = gen_type_guid(&self.0, gen);
        // TODO: can we share these or at least copy the resulting strings instead? Maybe if they're not iterators the quote macro won't consume them?
        let struct_phantoms = gen_phantoms(&self.0);
        let abi_phantoms = gen_phantoms(&self.0);
        let vtable_phantoms = gen_phantoms(&self.0);
        let constraints = gen_constraints(&self.0);

        let method = MethodInfo {
            name: "Invoke".to_string(),
            vtable_offset: 3,
            overload: 0,
            is_deprecated: false,
        };

        let interface = InterfaceInfo {
            def: self.0.clone(),
            kind: InterfaceKind::Default,
            is_base: false,
            version: (0, 0),
        };

        let invoke = gen_winrt_method(&signature, &method, &interface, gen);

        // This can't use TypeDef's type_signature method as this has to store the unspecialized guid
        // for compile-time const guid calculations.
        let type_signature = if self.0.generics.is_empty() {
            gen_signature(&self.0, &format!("delegate({{{:#?}}})", &self.0.guid()))
        } else {
            gen_signature(&self.0, &format!("{{{:#?}}}", &self.0.guid()))
        };

        let (box_name, box_definition) = if self.0.generics.is_empty() {
            let name = format_token!("{}_box", self.0.name());
            (quote! { #name::<F> }, quote! { #name<#fn_constraint> })
        } else {
            let name = self.0.name();
            let name = format_token!("{}_box", &name[..name.len() - 2]);
            let generics = self.0.generics.iter().map(|g| gen_name(g, gen));
            let generics = quote! { #(#generics,)* };
            (
                quote! { #name::<#generics F> },
                quote! { #name<#generics #fn_constraint> },
            )
        };

        let invoke_upcall = gen_winrt_upcall(&signature, quote! { ((*this).invoke) }, gen);

        quote! {
            #[repr(transparent)]
            #[derive(::std::cmp::PartialEq, ::std::cmp::Eq, ::std::clone::Clone, ::std::fmt::Debug)]
            pub struct #name(::windows::IUnknown, #(#struct_phantoms,)*) where #constraints;
            impl<#constraints> #name {
                pub fn new<#fn_constraint>(invoke: F) -> Self {
                    let com = #box_name {
                        vtable: &#box_name::VTABLE,
                        count: ::windows::RefCount::new(1),
                        invoke,
                    };
                    unsafe {
                        std::mem::transmute(::std::boxed::Box::new(com))
                    }
                }
                #invoke
            }
            unsafe impl<#constraints> ::windows::RuntimeType for #name {
                const SIGNATURE: ::windows::ConstBuffer = #type_signature;
            }
            unsafe impl<#constraints> ::windows::Interface for #name {
                type Vtable = #abi_name;
                const IID: ::windows::Guid = #guid;
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct #abi_name(
                pub unsafe extern "system" fn(this: ::windows::RawPtr, iid: &::windows::Guid, interface: *mut ::windows::RawPtr) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn #abi_signature,
                #(pub #abi_phantoms,)*
            ) where #constraints;
            #[repr(C)]
            struct #box_definition where #constraints {
                vtable: *const #abi_name,
                invoke: F,
                count: ::windows::RefCount,
            }
            impl<#constraints #fn_constraint> #box_name {
                const VTABLE: #abi_name = #turbo_abi_name(
                    Self::QueryInterface,
                    Self::AddRef,
                    Self::Release,
                    Self::Invoke,
                    #(#vtable_phantoms,)*
                );
                unsafe extern "system" fn QueryInterface(this: ::windows::RawPtr, iid: &::windows::Guid, interface: *mut ::windows::RawPtr) -> ::windows::HRESULT {
                    let this = this as *mut ::windows::RawPtr as *mut Self;

                    *interface = if iid == &<#name as ::windows::Interface>::IID ||
                        iid == &<::windows::IUnknown as ::windows::Interface>::IID ||
                        iid == &<::windows::IAgileObject as ::windows::Interface>::IID {
                            &mut (*this).vtable as *mut _ as _
                        } else {
                            ::std::ptr::null_mut()
                        };

                    // TODO: implement IMarshal

                    if (*interface).is_null() {
                        ::windows::HRESULT(0x8000_4002) // E_NOINTERFACE
                    } else {
                        (*this).count.add_ref();
                        ::windows::HRESULT(0)
                    }
                }
                unsafe extern "system" fn AddRef(this: ::windows::RawPtr) -> u32 {
                    let this = this as *mut ::windows::RawPtr as *mut Self;
                    (*this).count.add_ref()
                }
                unsafe extern "system" fn Release(this: ::windows::RawPtr) -> u32 {
                    let this = this as *mut ::windows::RawPtr as *mut Self;
                    let remaining = (*this).count.release();

                    if remaining == 0 {
                        Box::from_raw(this);
                    }

                    remaining
                }
                unsafe extern "system" fn Invoke #abi_signature {
                    let this = this as *mut ::windows::RawPtr as *mut Self;
                    #invoke_upcall
                }
            }
        }
    }
}
