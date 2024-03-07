pub trait Dump<TClient, TConnecrion> {
    async fn dump_database(
        &self,
        client: &TClient,
        connection: &TConnecrion,
        output: &str,
    ) -> Result<(), Option<i32>>;
}

pub trait DumpJob {
    async fn dump_databases(&self);
}
