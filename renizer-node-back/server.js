const express = require("express");
const { v4: uuidv4 } = require("uuid");
const { session, sessionStore } = require("./utils/session");

const auth = require("./routes/auth");

const app = express();
const PORT = 3000;

// middlewares
app.use(express.urlencoded({ extended: true }));
app.use(session({
    genid: () => uuidv4(),
    secret: process.env.REnizer_SESSION_SECRET,
    store: sessionStore,
    cookie: {
        maxAge: 7200000,
    },
    resave: false,
    saveUninitialized: false,
}))

// routes
app.use("/auth", auth);

app.get("/", (_, res) => {
    res.send({ message: "hello world!" });
});

app.listen(PORT, () => console.log(`listening on port ${PORT}`));