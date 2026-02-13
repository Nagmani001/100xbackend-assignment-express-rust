import { describe, expect, it } from "vitest";
import axios from "axios";

export const BASE_URL = "http://localhost:3000";

describe("GET / ", () => {
  it("should print welcome message ", async () => {
    const response = await axios.get(`${BASE_URL}/`);

    expect(response.status).toBe(200);
    expect(response.data).toEqual("Welcome to my first Express or Actix server!");
  });
});

describe("GET /greet ", () => {
  it("should greet the user with their name", async () => {
    const response = await axios.get(`${BASE_URL}/greet`, {
      params: {
        name: "Nagmani"
      }
    });
    expect(response.status).toBe(200);
    expect(response.data).toEqual("Hello Nagmani, nice to meet you!");
  });
});

describe("GET /write", () => {
  it("should write to a file", async () => {
    const response = await axios.get(`${BASE_URL}/write`, {
      params: {
        message: "Learning Express or actix"
      }
    });

    expect(response.status).toBe(200);
    expect(response.data).toEqual("wrote to file");
  });
});

describe("GET /append", () => {
  it("should append to a file", async () => {
    const response = await axios.get(`${BASE_URL}/append`, {
      params: {
        message: "this needs to be appended"
      }
    });

    expect(response.status).toBe(200);
    expect(response.data).toEqual("appended to file");
  });
});

describe("GET /read", () => {
  it("should read a file", async () => {
    const response = await axios.get(`${BASE_URL}/read`);
    expect(response.status).toBe(200);
    expect(response.data.trim()).toEqual("Learning Express or actixthis needs to be appended");
  });

});

describe("GET /clear", () => {
  it("should clear all the  contents of files", async () => {
    const response = await axios.get(`${BASE_URL}/clear`);

    expect(response.status).toBe(200);
    expect(response.data).toEqual("cleared");
  });
});


describe("GET /add-user", () => {
  it("should add user to uses array ", async () => {
    const response = await axios.get(`${BASE_URL}/add-user`, {
      params: {
        name: "Nagmani",
        age: 20
      }
    });

    expect(response.status).toBe(200);
    expect(response.data).toEqual("User added successfully!");
  });
});

describe("GET /check-user ", () => {
  it("blocks users with less age ", async () => {
    const response = await axios.get(`${BASE_URL}/check-user`, {
      params: {
        name: "Nagmani",
        age: 2
      }
    });

    expect(response.status).toBe(200);
    expect(response.data).toEqual("You are blocked as your age is less.");
  });

  it("allows user with proper age", async () => {
    const response = await axios.get(`${BASE_URL}/check-user`, {
      params: {
        name: "Raman",
        age: 20
      }
    });

    expect(response.status).toBe(200);
    expect(response.data).toEqual("Access granted!");
  });
});


describe("GET /is-blocked ", () => {

  it("should show blocked ", async () => {
    const name = "Nagmani"
    const response = await axios.get(`${BASE_URL}/is-blocked?name=${name}`);

    expect(response.status).toBe(200);
    expect(response.data).toEqual(`${name} is blocked.`);
  });

  it("should show not blocked", async () => {
    const name = "Raman"
    const response = await axios.get(`${BASE_URL}/is-blocked`, {
      params: {
        name
      }
    });

    expect(response.status).toBe(200);
    expect(response.data).toEqual(`${name} is not blocked.`);
  });
});

describe("GET /users ", () => {
  it("should show blocked ", async () => {
    const response = await axios.get(`${BASE_URL}/users`);

    expect(response.status).toBe(200);
    expect(response.data).toEqual([{ name: 'Nagmani', age: 20 }]);
  });
});

describe("DELETE /clear-data ", () => {
  it("should delete the in memory data ", async () => {
    const response = await axios.get(`${BASE_URL}/clear-data`);

    expect(response.status).toBe(200);
    expect(response.data).toEqual("All data cleared successfully!");
  });
});
