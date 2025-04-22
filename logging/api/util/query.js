import sql from "./db.js";

export async function getAllLogs(cursor, limit) {
    let logs;
    if (cursor) {
        logs = await sql`
            SELECT * FROM logs 
            WHERE id < ${cursor}
            ORDER BY id DESC
            LIMIT BY ${limit}
        `;
    } else {
        logs = await sql`SELECT * FROM logs ORDER BY id DESC LIMIT ${limit}`;
    }
    const nextCursor = logs.length ? logs[logs.length - 1].id : null;
    return {
        data: logs,
        nextCursor: nextCursor,
    };
}

export async function getLogsBetween(startDate, endDate, cursor, level = "") {
    return true;
}

export async function insertTestLog() {
    const logs = await sql`
        INSERT INTO logs 
        (level,message,target) 
        VALUES 
        ('info','some message','THE_target')
    `;
    return logs;
}
