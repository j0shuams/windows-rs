fn main() {
    windows::build! {
        Windows::Foundation::Collections::{IIterable, IVectorView, IMapView},
        Windows::Foundation::{IClosable, IStringable, Uri},
        Windows::Win32::Foundation::E_BOUNDS,
        Windows::Win32::System::WinRT::{IDisplayPathInterop, ISwapChainInterop},
        Windows::UI::Xaml::{Application, Controls::Button},
    };
}
