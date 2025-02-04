// SNOS is unable to process block 0, so we need to manually add an entry to the database to simulate 
// that block 0 has been settled into L1.
// This is a necessary step to allow the orchestrator to continue processing blocks, 
// as it would otherwise become stuck.

db = connect("mongodb://mongodb:27017/orchestrator");

db.jobs.insertOne({
    _id: ObjectId("6752ccc937d965f72eff206d"),
    internal_id: "0",
    job_type: "StateTransition",
    created_at: ISODate("2024-12-06T10:07:05.000Z"),
    external_id: "0",
    id: BinData(4, "H1c+V6J/TCaCID2LwJ5e/g=="),
    metadata: {
        attempt_tx_hashes_0: "0x0adca7145e618564bc5ec901b80d331e11e3207ac21e68c4cedb698ff5ce6cb0",
        process_attempt_no: "1",
        blocks_number_to_settle: "0"
    },
    status: "Completed",
    updated_at: ISODate("2024-12-06T10:18:20.000Z"),
    version: 3
});
