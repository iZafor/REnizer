const express = require("express");
const bcrypt = require("bcrypt");
const { pool } = require("../utils/database");

const router = express.Router();

router.post("/login", (req, res) => {
    let { email, password } = req.body;
    if (email === undefined || password === undefined) {
        res.status(401).send({ error: "undefined field" });
        return;
    }
    
    pool.query(
        "SELECT * FROM User_T WHERE email = ?",
        email,
        (err, result) => {
            if (err) {
                res.status(500).send({ error: err });
            } else {
                if (result.length == 0) {
                    res.status(200).send({ error: "invalid credentials" });
                    return;
                }

                req.session.user = result[0].user_id;
                req.session.regenerate(async (err) => {
                    if (err) {
                        res.status(500).send({ error: "session regeneration failed" });
                        return;
                    }

                    let match = await bcrypt.compare(password, result[0].password);
                    if (match) {
                        res.status(200).send({ user: result[0] });
                    } else {
                        res.status(200).send({ error: "invalid credentials" });
                    }
                })
            }
        }
    );
});

router.get("/logout", (req, res) => {
    req.session.user = null;
    req.session.regenerate((err) => {
        if (err) {
            res.status(500).send({ error: err });
        } else {
            res.status(200).send({ message: "logout successful" });
        }
    })
});

module.exports = router;
