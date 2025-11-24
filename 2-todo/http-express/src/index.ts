import express, { Request, Response } from "express";

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

app.get("/add-todo", (req: Request, res: Response) => {
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
    res.status(404).json({ error: "TODO not found" });
  }
  res.json(todo);
});

app.put("/todos/:id/complete", (req: Request, res: Response) => {
  const id = req.params.id;

  if (!id) return;
  const todo = todos.find(x => x.id == parseInt(id));
  if (!todo) {
    return res.status(404).json({ error: "TODO not found" });

  }

  todo.completed = true;

  res.json({ message: "TODO marked as completed!", todo: { id: 1, task: "Buy milk", completed: true } });
});

app.delete("/todos/:id", (req: Request, res: Response) => {
  const id = req.params.id;
  if (!id) return;
  const todo = todos.find(x => x.id == parseInt(id));
  if (!todo) {
    return res.status(404).json({ error: "TODO not found" });
  } else {
    console.log("todos before: ", todos);
    //TODO: filter not working as expected
    todos.filter(x => {
      return x.id !== parseInt(id);
    });
    console.log("todos after: ", todos);
    res.json({ message: "TODO deleted successfully!" });
  }
});

app.get("/todos/filter", (req: Request, res: Response) => {
  const status = req.query.status;

});


app.listen(3000, () => {
  console.log("server is running on port 3000");
});
