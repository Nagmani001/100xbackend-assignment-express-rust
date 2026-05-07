import axios from "axios";
import { describe, expect, it } from "vitest";

const BASE_URL = "http://localhost:3000";

describe("GET /", () => {
  it("welcome", async () => {
    const r = await axios.get(`${BASE_URL}/`);
    expect(r.status).toBe(200);
    expect(r.data).toEqual("Welcome to TODO Backend!");
  });
});

describe("GET /todos initially", () => {
  it("404 when no todos", async () => {
    try {
      await axios.get(`${BASE_URL}/todos`);
      throw new Error("should have thrown");
    } catch (e: any) {
      expect(e.response.status).toBe(404);
      expect(e.response.data).toStrictEqual({ message: "No TODOs found yet." });
    }
  });
});

describe("POST /add-todo", () => {
  it("adds todo", async () => {
    const r = await axios.post(`${BASE_URL}/add-todo`, null, {
      params: { task: "Buy milk" },
    });
    expect(r.status).toBe(201);
    expect(r.data).toStrictEqual({ message: "TODO added successfully!", todoCount: 1 });
  });
});

describe("GET /todos", () => {
  it("returns all todos", async () => {
    const r = await axios.get(`${BASE_URL}/todos`);
    expect(r.status).toBe(200);
    expect(r.data).toStrictEqual([{ id: 1, task: "Buy milk", completed: false }]);
  });
});

describe("GET /todo/:id", () => {
  it("returns todo by id", async () => {
    const r = await axios.get(`${BASE_URL}/todo/1`);
    expect(r.status).toBe(200);
    expect(r.data).toStrictEqual({ id: 1, task: "Buy milk", completed: false });
  });

  it("404 when not found", async () => {
    try {
      await axios.get(`${BASE_URL}/todo/2`);
      throw new Error("should have thrown");
    } catch (e: any) {
      expect(e.response.status).toBe(404);
      expect(e.response.data).toStrictEqual({ error: "TODO not found" });
    }
  });
});

describe("PUT /todos/:id/complete", () => {
  it("marks todo complete", async () => {
    const r = await axios.put(`${BASE_URL}/todos/1/complete`);
    expect(r.status).toBe(200);
    expect(r.data).toStrictEqual({
      message: "TODO marked as completed!",
      todo: { id: 1, task: "Buy milk", completed: true },
    });
  });

  it("404 when not found", async () => {
    try {
      await axios.put(`${BASE_URL}/todos/2/complete`);
      throw new Error("should have thrown");
    } catch (e: any) {
      expect(e.response.status).toBe(404);
      expect(e.response.data).toStrictEqual({ error: "TODO not found" });
    }
  });
});

describe("GET /todos/filter", () => {
  it("completed status returns completed todos", async () => {
    const r = await axios.get(`${BASE_URL}/todos/filter`, {
      params: { status: "completed" },
    });
    expect(r.status).toBe(200);
    expect(r.data).toStrictEqual([{ id: 1, task: "Buy milk", completed: true }]);
  });

  it("pending status returns pending todos", async () => {
    const r = await axios.get(`${BASE_URL}/todos/filter`, {
      params: { status: "pending" },
    });
    expect(r.status).toBe(200);
    expect(r.data).toStrictEqual([]);
  });
});

describe("DELETE /todos/:id", () => {
  it("deletes the todo", async () => {
    const r = await axios.delete(`${BASE_URL}/todos/1`);
    expect(r.status).toBe(200);
    expect(r.data).toStrictEqual({ message: "TODO deleted successfully!" });
  });

  it("404 after deletion", async () => {
    try {
      await axios.delete(`${BASE_URL}/todos/1`);
      throw new Error("should have thrown");
    } catch (e: any) {
      expect(e.response.status).toBe(404);
      expect(e.response.data).toStrictEqual({ error: "TODO not found" });
    }
  });
});
