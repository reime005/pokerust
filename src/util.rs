#[macro_export]
macro_rules! set_endpoint {
    { $A:ty, $B:literal } => {
impl crate::Endpoint for $A {
    const ENDPOINT: &'static str = $B;
}
};
}

#[macro_export]
macro_rules! impl_id {
    { $A:ty } => {
impl crate::Id for $A {
    fn id(&self) -> &i64 {
        &self.id
    }
}
};
}

#[macro_export]
macro_rules! impl_named {
    { $A:ty } => {
impl crate::Named for $A {
    fn name(&self) -> &String {
        &self.name
    }
}
};
}

#[macro_export]
macro_rules! impl_id_and_named {
    { $A:ty } => {
crate::impl_named!{$A}
crate::impl_id!{$A}
};
}
