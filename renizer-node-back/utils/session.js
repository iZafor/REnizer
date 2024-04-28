const session = require("express-session");
const MysqlStore = require("express-mysql-session")(session);
const { pool } = require("./database");

const sessionStore = new MysqlStore({
    expiration: 7200000,
    clearExpired: true,
    createDatabaseTable: true,
    schema: {
        tableName: "express_sessions",
        columnNames: {
            session_id: "session_id",
            expires: "expires",
            data: "data"
        }
    }
}, pool);

sessionStore.onReady().then((err) => {
    console.log("session store is ready")
}).catch((err) => {
    console.error(err);
});

module.exports = {
    session,
    sessionStore
};

