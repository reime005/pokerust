#[macro_export]
macro_rules! impl_from_id_and_name {
    { $A:ty, $B:literal } => {
impl FromId for $A {
    fn from_id(id: u64) -> Result<Self, minreq::Error> {
        get_resource(&format!(concat!($B, "/{}/"), id))?.json::<Self>()
    }
}

impl FromName for $A {
    fn from_name(name: &str) -> Result<Self, minreq::Error> {
        get_resource(&format!(concat!($B, "/{}/"), name))?.json::<Self>()
    }
}
};
}
