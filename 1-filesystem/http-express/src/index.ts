import fs from "fs";
import express, { Request, Response } from "express";

const app = express();

app.use(express.json());


app.get("/", (req: Request, res: Response) => {
  res.send("Welcome to my first Express or Actix server!")
});


app.get("/greet", (req: Request, res: Response) => {
  const name = req.query.name;
  res.send(`Hello ${name}, nice to meet you!`)
});

app.get("/write", (req: Request, res: Response) => {
  const dataToWrite = req.query.message;
  if (!dataToWrite) return;
  fs.writeFile("notes.txt", dataToWrite.toString(), (err: any) => {
    if (!err) {
      res.send("wrote to file");
    }
  });
});

app.get("/append", (req: Request, res: Response) => {
  const dataToAppend = req.query.message;
  if (!dataToAppend) return;
  fs.appendFile("notes.txt", dataToAppend.toString(), (err: any) => {
    if (!err) {
      res.send("appended to file");
    }
  });
});


app.get("/read", (req: Request, res: Response) => {
  fs.readFile("notes.txt", (err, data) => {
    if (err) {
      return res.send("No notes found yet.");
    }
    res.send(data);
  });
});


app.get("/clear", (req: Request, res: Response) => {
  fs.writeFile("notes.txt", "", (err) => {
    if (!err) {
      res.send("cleared");
    }
  });
});

interface User {
  name: string,
  age: number
}

let users: User[] = [];

app.get("/add-user", (req: Request, res: Response) => {
  const name = req.query.name as string;
  const age = req.query.age as string;


  users.push({
    name,
    age: parseInt(age)
  });
  res.send("User added successfully!");
});

let blockedUsers: any = {};

app.get("/check-user", (req: Request, res: Response) => {
  const name = req.query.name as string;
  const age = req.query.age as string;

  if (parseInt(age) < 18) {
    blockedUsers[name] = true;
    return res.send("You are blocked as your age is less.");
  } else {
    blockedUsers[name] = false;
    return res.send("Access granted!");
  }
});

app.get("/is-blocked", (req: Request, res: Response) => {
  const name = req.query.name as string;
  if (blockedUsers[name]) {
    res.send(`${name} is blocked.`);
  } else {
    res.send(`${name} is not blocked.`);
  }
});

app.get("/users", (req: Request, res: Response) => {
  res.json(users);
});

app.get("/clear-data", (req: Request, res: Response) => {
  users = [];
  blockedUsers = [];
  res.send("All data cleared successfully!");
});

app.listen(3000, () => {
  console.log("Server runnin on port 3000");
});
