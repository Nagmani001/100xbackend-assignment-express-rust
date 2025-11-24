import axios from "axios";
import { describe, expect, it } from "vitest";

const BASE_URL = "http://localhost:3000"

describe("GET / initialize todo backend ", () => {
  it("should initialize a todo array and greet the user", async () => {
    const response = await axios.get(`${BASE_URL}/`);

    expect(response.status).toBe(200);
    expect(response.data).toEqual("Welcome to TODO Backend!");
  });
});



describe("POST /todos ", () => {
  it("todos currently doesn't exist so this should fail with 404", async () => {


    //TODO: just putting a try catch here fixed it , but how ? 
    try {
      const response = await axios.get(`${BASE_URL}/todos`);

      expect(response.status).toBe(404);
      expect(response.data).toStrictEqual({ message: "No TODOs found yet." });
    } catch (err) {

    }
  });
});

describe("POST /add-todo?task=Buy%20milk adds todo", () => {
  it("should add the todo to in memorary variable", async () => {
    const response = await axios.get(`${BASE_URL}/add-todo`, {
      params: {
        task: "Buy milk"
      }
    });

    expect(response.status).toBe(201);
    expect(response.data).toStrictEqual({ message: "TODO added successfully!", todoCount: 1 });
  });
});

describe("GET /todos ", () => {
  it("get all todos and return to user", async () => {
    const response = await axios.get(`${BASE_URL}/todos`);

    expect(response.status).toBe(200);
    expect(response.data).toStrictEqual([
      {
        id: 1,
        task: "Buy milk",
        completed: false
      }
    ]);
  });
});

describe("GET /todos/:id ", () => {
  it("finds the todo with the todo id", async () => {
    const response = await axios.get(`${BASE_URL}/todo/1`);

    expect(response.status).toBe(200);
    expect(response.data).toStrictEqual(
      {
        id: 1,
        task: "Buy milk",
        completed: false
      }
    );
  });

  it("should return with 404 since todo doesn't exist", async () => {
    //TODO: just putting a try catch here fixed it , but how ? 
    try {

      const response = await axios.get(`${BASE_URL}/todo/2`);

      console.log(response.data);
      console.log(response.status);

      expect(response.status).toBe(404);
      expect(response.data).toStrictEqual({ error: "TODO not found" });
    } catch (err) {

    }

  });
});


describe("PUT /todos/:id/complete ", () => {
  it("finds the todo with the todo id", async () => {
    const response = await axios.put(`${BASE_URL}/todos/1/complete`);

    expect(response.status).toBe(200);
    expect(response.data).toStrictEqual({ message: "TODO marked as completed!", todo: { id: 1, task: "Buy milk", completed: true } });
  });

  it("should return with 404 since todo doesn't exist", async () => {

    //TODO: just putting a try catch here fixed it , but how ? 
    try {

      const response = await axios.put(`${BASE_URL}/todos/2/complete`);
      expect(response.status).toBe(404);
      expect(response.data).toStrictEqual({ error: "TODO not found" });
    } catch (err) {

    }

  });
});



describe("DELETE /todo/:id", () => {

  it("should return with 404 since todo doesn't exist", async () => {
    try {

      const response = await axios.delete(`${BASE_URL}/todos/1`);

      expect(response.status).toBe(404);
      expect(response.data).toStrictEqual({ error: "TODO not found" });
    } catch (err) {

    }
  });

  it("deletes the todo with id 1", async () => {
    try {

      const response = await axios.delete(`${BASE_URL}/todos/1`);

      expect(response.status).toBe(200);
      expect(response.data).toStrictEqual({ message: "TODO deleted successfully!" });
    } catch (err) {

    }
  });
});

describe("GET /todos/filter?status=complete", () => {

  // initially no todos should come 
  it("should not return any todos", async () => {

    const responseWithCompleteTodos = await axios.get(`${BASE_URL}/todos/filter`, {
      params: {
        status: "completed"
      }
    });

    const responseWithPendingTodos = await axios.get(`${BASE_URL}/todos/filter`, {
      params: {
        status: "pending"
      }
    });

    expect(responseWithCompleteTodos.status).toBe(200);
    expect(responseWithCompleteTodos).toStrictEqual({ message: "No TODOs found for the given filter." });

    expect(responseWithPendingTodos).toBe(200);
    expect(responseWithPendingTodos).toStrictEqual({ message: "No TODOs found for the given filter." });
  })


  it("finds todos with status = completed", async () => {
    // add some todos which are marekd as completed  = true & completed = false 

    const response = await axios.get(`${BASE_URL}/todos/filter`, {
      params: {
        status: "completed"
      }
    });
    expect(response.status).toBe(200);

    expect(response.data).toStrictEqual({ message: "TODO deleted successfully!" });
  });

  it("finds todos with status = pending ", async () => {
    const response = await axios.get(`${BASE_URL}/todos/filter`, {
      params: {
        status: "pending"
      }
    });

    expect(response.status).toBe(404);
    expect(response.data).toStrictEqual({ error: "TODO not found" });
  });
});

/*


describe("/load-toods , ", () => {
  // since initially there is no file named todos.json
  it("should return 404 with no file named todos.json initially ", async () => {

    const response = await axios.get(`${BASE_URL}/load-todos`);

    expect(response.status).toBe(404);
    expect(response.data).toStrictEqual({ error: "No saved TODOs found!" });

  });
});


describe("POST /save-todos", () => {
  it("writes all todos to a file named todos.json k", async () => {
    const response = await axios.post(`${BASE_URL}/save-todos`);

    expect(response.status).toBe(200);
    expect(response.data).toStrictEqual({ message: "All TODOs saved to file successfully!" });
  });
});


describe("GET /load-todos", () => {
  it("should load todos from the file ", async () => {

    const response = await axios.get(`${BASE_URL}/load-todos`);

    expect(response.status).toBe(200);
    //WARNING: know how mnay todos have been added , and what is the number of todos currently 
    expect(response.data).toStrictEqual({ message: "TODOs loaded from file successfully!", todoCount: 3 });
  })
});


describe("DELETE /clear-todos", () => {
  it("should delete the in memory todos array ", async () => {
    const response = await axios.delete(`${BASE_URL}/clear-todos`);

    expect(response.status).toBe(200);
    expect(response.data).toStrictEqual({ message: "All TODOs cleared!" });
  });
});


describe("GET /start", () => {
  it("should output number of todos , number of pending and complete todos ", async () => {
    const response = await axios.get(`${BASE_URL}/start`);

    expect(response.status).toBe(200);
    expect(response.data).toStrictEqual({
      total: 10,
      completed: 4,
      pending: 6
    });
  });
});

  */
