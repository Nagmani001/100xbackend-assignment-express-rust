import express, { Request, Response } from "express";
import fs from "fs";

const app = express();

app.use(express.json());


interface Todo {
  id: number,
  task: string,
  completed: boolean
}

let todos: any[] = [];
let GLOBAL_ID = 1;

app.get("/", (req: Request, res: Response) => {
  res.send("Welcome to TODO Backend!");
});

app.post("/add-todo", (req: Request, res: Response) => {
  const task = req.query.task as string;
  let todo = {
    id: GLOBAL_ID,
    task: task,
    completed: false
  };

  todos.push(todo);
  GLOBAL_ID++;
  res.status(201).json({ message: "TODO added successfully!", todoCount: todos.length });
});

app.get("/todos", (req: Request, res: Response) => {
  if (todos.length === 0) {
    return res.status(404).json({ message: "No TODOs found yet." });
  }
  res.json(todos);
});

app.get("/todo/:id", (req: Request, res: Response) => {
  const id = req.params.id;
  if (!id) return;
  const todo = todos.find(x => x.id == parseInt(id));
  if (!todo) {
    return res.status(404).json({ error: "TODO not found" });
  }
  res.json(todo);
});

app.put("/todos/:id/complete", (req: Request, res: Response) => {
  const id = req.params.id as string;

  if (!id) return;
  const todo = todos.find(x => x.id == parseInt(id));
  if (!todo) {
    return res.status(404).json({ error: "TODO not found" });

  }

  todo.completed = true;

  res.json({ message: "TODO marked as completed!", todo });
});

app.delete("/todos/:id", (req: Request, res: Response) => {
  const id = req.params.id as string;
  if (!id) return;
  const todo = todos.find(x => x.id == parseInt(id));
  if (!todo) {
    return res.status(404).json({ error: "TODO not found" });
  } else {
    todos = todos.filter(x => x.id !== parseInt(id));
    res.json({ message: "TODO deleted successfully!" });
  }
});

app.get("/todos/filter", (req: Request, res: Response) => {
  const status = req.query.status;
  if (status == "completed") {
    const todo_complete = todos.filter(x => x.completed);
    res.json(todo_complete);

  } else if (status == "pending") {
    const todo_pending = todos.filter(x => !x.completed);
    res.json(todo_pending);

  } else {
    res.status(404).json({ message: "No TODOs found for the given filter." });
  }
});

app.post("/save-todos", (req: Request, res: Response) => {
  fs.writeFileSync("./todos.json", JSON.stringify(todos));
  res.json({ message: "All TODOs saved to file successfully!" });
});

app.post("/load-todos", (req: Request, res: Response) => {
  let todo_from_file = fs.readFileSync("./todos.json", "utf-8")
  if (!todo_from_file) {
    return res.status(404).json({ error: "No saved TODOs found!" });
  }
  todos = JSON.parse(todo_from_file);
  res.json({ message: "TODOs loaded from file successfully!", todoCount: todos.length });
});

app.delete("/clear-todos", (req: Request, res: Response) => {
  todos = [];
  res.json({ message: "All TODOs cleared!" });
});

app.get("/stats", (req: Request, res: Response) => {
  const completed_todos = todos.filter(x => x.completed);
  const incomplete_todos = todos.filter(x => !x.completed);

  res.json({
    total: todos.length,
    completed: completed_todos.length,
    pending: incomplete_todos.length
  });
});


app.listen(3000, () => {
  console.log("server is running on port 3000");
});
