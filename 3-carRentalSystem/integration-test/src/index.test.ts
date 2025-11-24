import axios from "axios";
import { describe, expect, it } from "vitest";


const BASE_URL = "http://localhost:3000";
describe("/POST /signup returns the userId", () => {
  it("should fail when sent invalid input ", async () => {
    const response = await axios.post(`${BASE_URL}/signup`, {
      username: "Nagmani",
      password: 21
    });
    expect(response.status).toBe(400);
    expect(response.data).toStrictEqual({
      message: "invalid data"
    });
  });

  it("should fail when trying to signup twice with the same username", async () => {
    const response1 = await axios.post(`${BASE_URL}/signup`, {
      username: "Nagmani",
      password: "itsboy"
    });

    expect(response1.status).toBe(201);
    expect(response1.data).toStrictEqual({ message: "User created successfully", userId: 1 });

    const response2 = await axios.post(`${BASE_URL}/signup`, {
      username: "Nagmani",
      password: "itsboy"
    });

    expect(response2.status).toBe(401);
    expect(response2.data).toStrictEqual({
      message: "user already exist"
    });
  });
});


describe("GET /users should return all the users", () => {
  it("should display all the users ", async () => {
    const response = await axios.get(`${BASE_URL}/users`);

    expect(response.status).toBe(200);
    expect(response.data).toStrictEqual({
      users: [{
        id: 1,
        username: "Nagmani",
        password: "itsboy",
        bookings: []
      }]
    });
  });
});

describe("POST /bookings/id", () => {
  it("should create a new booking", async () => {
    const resopnse = await axios.post(`${BASE_URL}/bookings/1`, {
      carName: "mustang",
      days: 2,
      rentPerDay: 20000
    });

    expect(resopnse.status).toBe(201);
    expect(resopnse.data).toStrictEqual({
      message: "mustang booked",
      bookingId: 101,
      totalCost: 40000
    });
  });
});

describe("GET /bookings/userId", () => {
  it("should return all the bookings of that usre", async () => {
    const response = await axios.get(`${BASE_URL}/bookings/1`);

    expect(response.status).toBe(200);
    expect(response.data).toStrictEqual({
      bookings: [{
        bookingId: 101,
        carName: "mustang",
        days: 2,
        rentPerDay: 20000,
        status: "booked",
      }]
    });
  });
});

describe("GET /bookings/:userId/:bookingId", () => {
  it("returns a specific booking of a specific user", async () => {
    const response = await axios.get(`${BASE_URL}/bookings/1/101`);

    expect(response.status).toBe(200);
    expect(response.data).toStrictEqual({
      bookingId: 101,
      carName: "mustang",
      days: 2,
      rentPerDay: 20000,
      status: "booked",
    });
  });

  it("returns with 404 not found ", async () => {
    const response = await axios.get(`${BASE_URL}/bookings/1/102`);

    expect(response.status).toBe(404);
    expect(response.data).toStrictEqual({
      message: "booking not found"
    });
  });
});

describe("PUT /bookings/:userId/:bookingId", () => {

});

describe("PUT /bookings/:userId/:bookingId/status", () => {
  it("", async () => {
    const response = await axios.put(`${BASE_URL}/bookings/1/101/status`, {
      status: "completed"
    });

    expect(response.status).toBe(200);
    expect(response.data()).toStrictEqual({ message: "Status updated successfully" });
  });
});

describe("DELETE /bookings/:userId/:bookingId", () => {
  it("deletes a specific booking for that user", async () => {

    const response = await axios.delete(`${BASE_URL}/bookings/1/101`);

    expect(response.status).toBe(200);
    expect(response.data).toStrictEqual({ message: "Booking deleted successfully" });
  });
});

describe("GET /summary/:userId", () => {
  it("shoud show the summary of a user ", async () => {
    const response = await axios.get(`${BASE_URL}/summary/1`)

    expect(response.status).toBe(200);
    expect(response.data).toStrictEqual({
      userId: 1,
      username: "Nagmani",
      totalBookings: 0,
      totalAmountSpent: 0
    });

  });
});
