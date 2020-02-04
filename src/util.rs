#[macro_export]
macro_rules! from_id_and_name {
    { $A:ty, $B:literal } => {
impl $A {
    pub fn from_id(id: u64) -> Result<Self, minreq::Error> {
        get_resource(&format!(concat!($B, "/{}/"), id))?.json::<Self>()
    }

    pub fn from_name(name: &str) -> Result<Self, minreq::Error> {
        get_resource(&format!(concat!($B, "/{}/"), name))?.json::<Self>()
    }
}
};
}
