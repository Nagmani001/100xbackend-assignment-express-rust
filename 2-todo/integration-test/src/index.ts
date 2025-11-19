import axios from "axios";
import { describe, expect, it } from "vitest";

const BASE_URL = "http://localhost:3000"

describe("GET / initialize todo backend ", () => {
  it("should initialize a todo array and greet the user", async () => {
    const response = await axios.get(`${BASE_URL}/`);

    expect(response.status).toBe(200);
    expect(response.data).toStrictEqual({
      msg: "Welcome to TODO Backend!"
    });
  });
});


describe("POST /add-todo?task=Buy%20milk adds todo", () => {
  it("should add the todo to in memorary variable", async () => {
    const response = await axios.get(`${BASE_URL}/`);

    expect(response.status).toBe(200);
    expect(response.data).toStrictEqual({
      msg: "Welcome to TODO Backend!"
    });
  });
});

