pub trait JsonataData {
    fn get_field(&self, field: &str) -> Option<Self>
        where 
            Self: Sized;

    fn as_f64(&self) -> Option<f64>;

    fn from_f64(value: f64) -> Self
        where Self: Sized;
    
    fn is_array(&self) -> bool;

    fn as_array(&self) -> Option<Vec<Self>>
        where Self: Sized;

    fn from_array(array: Vec<Self>) -> Self
        where Self: Sized;
}


