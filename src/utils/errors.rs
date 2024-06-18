pub struct ErrorResponse<T> {
    error: T,
    error_type: BlueberryApiErrors,
}

pub enum BlueberryApiErrors {}
