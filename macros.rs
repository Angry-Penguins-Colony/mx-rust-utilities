/// Equivalent to sc_panic! but I can specify the API
#[macro_export]
macro_rules! sc_panic_self {
    ($api: ty, $msg:tt, $($arg:expr),+ $(,)?) => {{
        let mut ___buffer___ =
            elrond_wasm::types::ManagedBufferCachedBuilder::<$api>::new_from_slice(&[]);
        elrond_wasm::derive::format_receiver_args!(___buffer___, $msg, $($arg),+);
        elrond_wasm::contract_base::ErrorHelper::<$api>::signal_error_with_message(___buffer___.into_managed_buffer());
    }};
    ($api: ty, $msg:expr $(,)?) => {
        elrond_wasm::contract_base::ErrorHelper::<$api>::signal_error_with_message($msg)
    };
}
