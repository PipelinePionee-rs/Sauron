import "dotenv/config";
import express from "express";
import { getAllLogs, insertTestLog } from "./util/query.js";
const app = express();

import logsRouter from "./routers/logsRouter.js";
app.use(logsRouter);

console.log(await insertTestLog());
console.log(await getAllLogs("", 50));

const PORT = process.env.PORT || 8080;

app.listen(PORT, () => console.log(`Server listening on ${PORT}`));
