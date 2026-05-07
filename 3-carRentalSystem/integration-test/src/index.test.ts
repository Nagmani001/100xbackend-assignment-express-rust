import axios from "axios";
import { describe, expect, it } from "vitest";

const BASE_URL = "http://localhost:3000";
const client = axios.create({ baseURL: BASE_URL, validateStatus: () => true });

describe("POST /signup", () => {
  it("400 invalid input", async () => {
    const r = await client.post("/signup", { username: "Nagmani", password: 21 });
    expect(r.status).toBe(400);
    expect(r.data).toStrictEqual({ message: "invalid data" });
  });

  it("201 first signup, 401 duplicate", async () => {
    const r1 = await client.post("/signup", { username: "Nagmani", password: "itsboy" });
    expect(r1.status).toBe(201);
    expect(r1.data).toStrictEqual({ message: "User created successfully", userId: 1 });

    const r2 = await client.post("/signup", { username: "Nagmani", password: "itsboy" });
    expect(r2.status).toBe(401);
    expect(r2.data).toStrictEqual({ message: "user already exist" });
  });
});

describe("GET /users", () => {
  it("returns all users", async () => {
    const r = await client.get("/users");
    expect(r.status).toBe(200);
    expect(r.data).toStrictEqual({
      users: [{ id: 1, username: "Nagmani", password: "itsboy", bookings: [] }],
    });
  });
});

describe("POST /bookings/:userId", () => {
  it("creates booking", async () => {
    const r = await client.post("/bookings/1", {
      carName: "mustang",
      days: 2,
      rentPerDay: 20000,
    });
    expect(r.status).toBe(201);
    expect(r.data).toStrictEqual({
      message: "mustang booked",
      bookingId: 101,
      totalCost: 40000,
    });
  });
});

describe("GET /bookings/:userId", () => {
  it("returns user bookings", async () => {
    const r = await client.get("/bookings/1");
    expect(r.status).toBe(200);
    expect(r.data).toStrictEqual({
      bookings: [
        {
          bookingId: 101,
          carName: "mustang",
          days: 2,
          rentPerDay: 20000,
          status: "booked",
        },
      ],
    });
  });
});

describe("GET /bookings/:userId/:bookingId", () => {
  it("returns specific booking", async () => {
    const r = await client.get("/bookings/1/101");
    expect(r.status).toBe(200);
    expect(r.data).toStrictEqual({
      bookingId: 101,
      carName: "mustang",
      days: 2,
      rentPerDay: 20000,
      status: "booked",
    });
  });

  it("404 not found", async () => {
    const r = await client.get("/bookings/1/102");
    expect(r.status).toBe(404);
    expect(r.data).toStrictEqual({ message: "booking not found" });
  });
});

describe("PUT /bookings/:userId/:bookingId/status", () => {
  it("updates status", async () => {
    const r = await client.put("/bookings/1/101/status", { status: "completed" });
    expect(r.status).toBe(200);
    expect(r.data).toStrictEqual({ message: "Status updated successfully" });
  });
});

describe("DELETE /bookings/:userId/:bookingId", () => {
  it("deletes booking", async () => {
    const r = await client.delete("/bookings/1/101");
    expect(r.status).toBe(200);
    expect(r.data).toStrictEqual({ message: "Booking deleted successfully" });
  });
});

describe("GET /summary/:userId", () => {
  it("returns user summary", async () => {
    const r = await client.get("/summary/1");
    expect(r.status).toBe(200);
    expect(r.data).toStrictEqual({
      userId: 1,
      username: "Nagmani",
      totalBookings: 0,
      totalAmountSpent: 0,
    });
  });
});
