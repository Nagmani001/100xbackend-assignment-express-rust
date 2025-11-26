import axios from "axios";
import { describe, expect, it } from "vitest";

const BASE_URL = "http://localhost:3000";

describe("POST /signup signup endpoint", () => {
  it("should let the user signup", async () => {
    const resopnse = await axios.post(`${BASE_URL}/signup`, {
      username: "Nagmani",
      email: "nagmanipd3@gmail.com",
      password: "itsbboy"
    });

    expect(resopnse.status).toBe(201);
    expect(resopnse.data).toStrictEqual({
      message: "User created successfully",
      userId: 1
    });
  });

  it("shouldn't let the user signup on invalid input", async () => {
    const resopnse = await axios.post(`${BASE_URL}/signup`, {
      username: "Nagmani",
      email: "nagmanipail.com",
      password: "itsbboy"
    });

    expect(resopnse.status).toBe(400);
    expect(resopnse.data).toStrictEqual({
      message: "invalid input",
    });
  });

  it("shouldn't let the user signup on email which already exists", async () => {
    const resopnse = await axios.post(`${BASE_URL}/signup`, {
      username: "Nagmani",
      email: "nagmanipd3@gmail.com",
      password: "itsbboasdfasdfy"
    });

    expect(resopnse.status).toBe(401);
    expect(resopnse.data).toStrictEqual({
      message: "user already exists",
    });
  });
});

describe("GET /movies", () => {
  it("get all movies ", async () => {
    const response = await axios.get(`${BASE_URL}/movies`);
    expect(response.status).toBe(200);
    expect(response.data).toStrictEqual({
      movies: [
        {
          "id": 1,
          "title": "Inception",
          "genre": "Sci-Fi",
          "duration": 148,
          "shows": [
            { "showId": 101, "time": "10:00 AM", "pricePerSeat": 200, "availableSeats": 50 },
            { "showId": 102, "time": "2:00 PM", "pricePerSeat": 250, "availableSeats": 50 },
            { "showId": 103, "time": "6:00 PM", "pricePerSeat": 300, "availableSeats": 50 }
          ]
        },
        {
          "id": 2,
          "title": "The Dark Knight",
          "genre": "Action",
          "duration": 152,
          "shows": [
            { "showId": 201, "time": "11:00 AM", "pricePerSeat": 200, "availableSeats": 50 },
            { "showId": 202, "time": "3:00 PM", "pricePerSeat": 250, "availableSeats": 50 },
            { "showId": 203, "time": "7:00 PM", "pricePerSeat": 300, "availableSeats": 50 }
          ]
        },
        {
          "id": 3,
          "title": "Interstellar",
          "genre": "Sci-Fi",
          "duration": 169,
          "shows": [
            { "showId": 301, "time": "12:00 PM", "pricePerSeat": 250, "availableSeats": 50 },
            { "showId": 302, "time": "5:00 PM", "pricePerSeat": 300, "availableSeats": 50 }
          ]
        }
      ]
    });
  });
});

describe("GET /movies/:movieId", () => {
  it("Return details of a specific movie including all its shows", async () => {
    const response = await axios.get(`${BASE_URL}/moview/1`);

    expect(response.status).toBe(200);
    expect(response.data).toStrictEqual(
      {
        "id": 1,
        "title": "Inception",
        "genre": "Sci-Fi",
        "duration": 148,
        "shows": [
          { "showId": 101, "time": "10:00 AM", "pricePerSeat": 200, "availableSeats": 50 },
          { "showId": 102, "time": "2:00 PM", "pricePerSeat": 250, "availableSeats": 50 },
          { "showId": 103, "time": "6:00 PM", "pricePerSeat": 300, "availableSeats": 50 }
        ]
      });
  });

  it("shouldn't Return details of a specific movie including all its shows", async () => {
    const response = await axios.get(`${BASE_URL}/moview/100`);

    expect(response.status).toBe(200);
    expect(response.data).toStrictEqual({ "message": "Movie not found" });
  });
});

describe("GET /movies/:movieId/shows", () => {
  it("Return only the shows of a specific movie", async () => {
    const response = await axios.get(`${BASE_URL}/movies/1/shows`);
    expect(response.status).toBe(200);
    expect(response.data).toStrictEqual({
      shows: [
        { "showId": 101, "time": "10:00 AM", "pricePerSeat": 200, "availableSeats": 50 },
        { "showId": 102, "time": "2:00 PM", "pricePerSeat": 250, "availableSeats": 50 },
        { "showId": 103, "time": "6:00 PM", "pricePerSeat": 300, "availableSeats": 50 }
      ]
    });
  });
});

describe("POST /bookings/:userId", () => {
  it("creates a new booking for the user", async () => {
    const response = await axios.post(`${BASE_URL}/bookings/1`, {
      movieId: "",
      showId: "",
      seats: ""
    })
  });
});


describe("GET /bookings/:userId/:bookingId", () => {
  it("Return details of a specific booking", async () => {
    const response = await axios.get(`${BASE_URL}/bookings/1/1001`);

    expect(response.status).toBe(200);
    expect(response.data).toStrictEqual({});
  });
});

describe("PUT /bookings/:userId/:bookingId", () => {
  it("", async () => {
    const response = await axios.post(`${BASE_URL}/bookings/1/1001`, {

    });

  })
});


describe("DELETE /bookings/:userId/:bookingId", () => {
  it("", async () => {
    const response = await axios.post(`${BASE_URL}/bookings/1/1001`, {

    });
  })
});


describe("GET /summary/:userId", () => {
  it("", async () => {
    const response = await axios.post(`${BASE_URL}/bookings/1/1001`, {

    });
  })
});
