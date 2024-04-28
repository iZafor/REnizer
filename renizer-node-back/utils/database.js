const mysql = require("mysql2");

const pool = mysql.createPool({
    host: process.env.REnizer_DB_HOST,
    port: process.env.REnizer_DB_PORT,
    user: process.env.REnizer_DB_USER_NAME,
    password: process.env.REnizer_DB_USER_PASSWORD,
    database: "REnizer",
});

pool.getConnection((err) => {
    if (err) {
        console.error(err);
    } else {
        console.log("db connected");
    }
});

module.exports = { pool };