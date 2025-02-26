// SNOS is unable to process block 0 ~ X, where X is the block where Bootstrapper setup-l2 finishes.
// So we need to manually add an entry to the database to simulate
// that these blocks are settled in the L1.
// This is a necessary step to allow the orchestrator to continue processing blocks,
// as it would otherwise become stuck.

async function insertStateTransition() {
    // Fetch madara last block
    const LAST_BLOCK_NUMBER = await fetch('http://madara:9945/v0_7_1/', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({
            "jsonrpc": "2.0",
            "method": "starknet_blockNumber",
            "params": [],
            "id": 1
        }),
    })
        .then(response => response.json())
        .then(data => data.result);

    console.log("Madara last block: {}", LAST_BLOCK_NUMBER)

    for (let i = 0; i <= LAST_BLOCK_NUMBER; i++) {
        db.jobs.insertOne({
            _id: ObjectId(),
            internal_id: i.toString(),
            job_type: "StateTransition",
            created_at: ISODate(`2024-12-06T10:07:05.000Z`),
            external_id: i.toString(),
            id: BinData(4, `H1c+V6J/TCaCID2LwJ5e/g==0`),
            metadata: {
                attempt_tx_hashes_0: `0x0adca7145e618564bc5ec901b80d331e11e3207ac21e68c4cedb698ff5ce6cb0`,
                process_attempt_no: "1",
                blocks_number_to_settle: i.toString()
            },
            status: "Completed",
            updated_at: ISODate(`2024-12-06T10:18:20.000Z`),
            version: 3
        });
    }
}

db = connect("mongodb://mongodb:27017/orchestrator");

// Let's clear all existing entries from the jobs collection
// This will make orchestrator start from scratch (SnosRun, ProofCreation, DataSubmission, etc)
// and we don't need to manually delete the entries from MongoDB
db.jobs.deleteMany({});

// Insert state transition completed until last Madara block
insertStateTransition();