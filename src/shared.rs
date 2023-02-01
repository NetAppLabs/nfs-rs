/// Struct describing an NFS timestamp.
#[derive(Debug, Default, PartialEq)]
pub struct Time {
    pub seconds: u32,
    pub nseconds: u32,
}
