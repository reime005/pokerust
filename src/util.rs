#[macro_export]
macro_rules! impl_from_id_and_name {
    { $A:ty, $B:literal } => {
impl crate::FromId for $A {
    fn from_id(id: u64) -> Result<Self, ::minreq::Error> {
        crate::cache::get_resource(&format!(concat!($B, "/{}/"), id))?.json::<Self>()
    }
}

impl crate::FromName for $A {
    fn from_name(name: &str) -> Result<Self, ::minreq::Error> {
        crate::cache::get_resource(&format!(concat!($B, "/{}/"), name))?.json::<Self>()
    }
}
};
}
