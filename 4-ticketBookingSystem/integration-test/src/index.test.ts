import axios from "axios";
import { describe, expect, it } from "vitest";

const BASE_URL = "http://localhost:3000";
const c = axios.create({ baseURL: BASE_URL, validateStatus: () => true });

const movies = [
  {
    id: 1,
    title: "Inception",
    genre: "Sci-Fi",
    duration: 148,
    shows: [
      { showId: 101, time: "10:00 AM", pricePerSeat: 200, availableSeats: 50 },
      { showId: 102, time: "2:00 PM", pricePerSeat: 250, availableSeats: 50 },
      { showId: 103, time: "6:00 PM", pricePerSeat: 300, availableSeats: 50 },
    ],
  },
  {
    id: 2,
    title: "The Dark Knight",
    genre: "Action",
    duration: 152,
    shows: [
      { showId: 201, time: "11:00 AM", pricePerSeat: 200, availableSeats: 50 },
      { showId: 202, time: "3:00 PM", pricePerSeat: 250, availableSeats: 50 },
      { showId: 203, time: "7:00 PM", pricePerSeat: 300, availableSeats: 50 },
    ],
  },
  {
    id: 3,
    title: "Interstellar",
    genre: "Sci-Fi",
    duration: 169,
    shows: [
      { showId: 301, time: "12:00 PM", pricePerSeat: 250, availableSeats: 50 },
      { showId: 302, time: "5:00 PM", pricePerSeat: 300, availableSeats: 50 },
    ],
  },
];

describe("POST /signup", () => {
  it("creates user", async () => {
    const r = await c.post("/signup", {
      username: "Nagmani",
      email: "nagmanipd3@gmail.com",
      password: "itsbboy",
    });
    expect(r.status).toBe(201);
    expect(r.data).toStrictEqual({ message: "User created successfully", userId: 1 });
  });

  it("400 invalid email", async () => {
    const r = await c.post("/signup", {
      username: "Nagmani",
      email: "nagmanipail.com",
      password: "itsbboy",
    });
    expect(r.status).toBe(400);
    expect(r.data).toStrictEqual({ message: "invalid input" });
  });

  it("401 duplicate email", async () => {
    const r = await c.post("/signup", {
      username: "Nagmani",
      email: "nagmanipd3@gmail.com",
      password: "itsbboasdfasdfy",
    });
    expect(r.status).toBe(401);
    expect(r.data).toStrictEqual({ message: "user already exists" });
  });
});

describe("GET /movies", () => {
  it("returns all movies", async () => {
    const r = await c.get("/movies");
    expect(r.status).toBe(200);
    expect(r.data).toStrictEqual({ movies });
  });
});

describe("GET /movies/:movieId", () => {
  it("returns movie", async () => {
    const r = await c.get("/movies/1");
    expect(r.status).toBe(200);
    expect(r.data).toStrictEqual(movies[0]);
  });

  it("404 when not found", async () => {
    const r = await c.get("/movies/100");
    expect(r.status).toBe(404);
    expect(r.data).toStrictEqual({ message: "Movie not found" });
  });
});

describe("GET /movies/:movieId/shows", () => {
  it("returns shows", async () => {
    const r = await c.get("/movies/1/shows");
    expect(r.status).toBe(200);
    expect(r.data).toStrictEqual({ shows: movies[0].shows });
  });
});

describe("POST /bookings/:userId", () => {
  it("creates booking", async () => {
    const r = await c.post("/bookings/1", { movieId: 1, showId: 101, seats: 5 });
    expect(r.status).toBe(201);
    expect(r.data).toStrictEqual({
      message: "Booking successful",
      bookingId: 1001,
      movieTitle: "Inception",
      showTime: "10:00 AM",
      seats: 5,
      totalAmount: 1000,
    });
  });
});

describe("GET /bookings/:userId/:bookingId", () => {
  it("returns specific booking", async () => {
    const r = await c.get("/bookings/1/1001");
    expect(r.status).toBe(200);
    expect(r.data).toMatchObject({
      bookingId: 1001,
      movieId: 1,
      showId: 101,
      seats: 5,
      totalAmount: 1000,
      status: "confirmed",
    });
  });
});

describe("PUT /bookings/:userId/:bookingId", () => {
  it("adds more seats", async () => {
    const r = await c.put("/bookings/1/1001", { seats: 2 });
    expect(r.status).toBe(200);
    expect(r.data).toStrictEqual({
      message: "Booking updated successfully",
      bookingId: 1001,
      seats: 7,
      totalAmount: 1400,
    });
  });
});

describe("DELETE /bookings/:userId/:bookingId", () => {
  it("cancels booking", async () => {
    const r = await c.delete("/bookings/1/1001");
    expect(r.status).toBe(200);
    expect(r.data).toStrictEqual({ message: "Booking cancelled successfully" });
  });
});

describe("GET /summary/:userId", () => {
  it("returns user summary", async () => {
    const r = await c.get("/summary/1");
    expect(r.status).toBe(200);
    expect(r.data).toStrictEqual({
      userId: 1,
      username: "Nagmani",
      totalBookings: 1,
      totalAmountSpent: 1400,
      confirmedBookings: 0,
      cancelledBookings: 1,
      totalSeatsBooked: 7,
    });
  });
});
