// MongoDB initialization script
// This script runs once when the MongoDB container is first created

db = db.getSiblingDB('lecture_metadata');

print("Creating indexes for lecture_metadata database...");

// Create unique index on module name
db.modules.createIndex(
    { "name": 1 },
    { unique: true, name: "unique_module_name" }
);
print("Created unique index on modules.name");

// Create unique index on lecture_id
db.lectures.createIndex(
    { "lecture_id": 1 },
    { unique: true, name: "unique_lecture_id" }
);
print("Created unique index on lectures.lecture_id");

// Optional: Create additional indexes for better query performance
db.lectures.createIndex(
    { "module_name": 1 },
    { name: "module_name_idx" }
);
print("Created index on lectures.module_name");

print("All indexes created successfully!");
