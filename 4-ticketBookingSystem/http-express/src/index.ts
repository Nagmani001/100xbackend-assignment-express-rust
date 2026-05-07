import express, { Request, Response } from "express";

const app = express();
app.use(express.json());

let GLOBAL_USER_ID = 1;
let GLOBAL_BOOKING_ID = 1001;

interface Show {
  showId: number;
  time: string;
  pricePerSeat: number;
  availableSeats: number;
}

interface Movie {
  id: number;
  title: string;
  genre: string;
  duration: number;
  shows: Show[];
}

interface Booking {
  bookingId: number;
  movieId: number;
  showId: number;
  seats: number;
  totalAmount: number;
  status: string;
}

interface User {
  id: number;
  username: string;
  email: string;
  password: string;
  bookings: Booking[];
}

const movies: Movie[] = [
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

const users: User[] = [];

const emailRe = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;

app.post("/signup", (req: Request, res: Response) => {
  const { username, email, password } = req.body || {};
  if (
    typeof username !== "string" ||
    typeof email !== "string" ||
    typeof password !== "string" ||
    !emailRe.test(email)
  ) {
    return res.status(400).json({ message: "invalid input" });
  }
  if (users.find((u) => u.email === email)) {
    return res.status(401).json({ message: "user already exists" });
  }
  const id = GLOBAL_USER_ID++;
  users.push({ id, username, email, password, bookings: [] });
  res.status(201).json({ message: "User created successfully", userId: id });
});

app.get("/movies", (_req: Request, res: Response) => {
  res.json({ movies });
});

app.get("/movies/:movieId", (req: Request, res: Response) => {
  const id = parseInt(req.params.movieId as string);
  const movie = movies.find((m) => m.id === id);
  if (!movie) return res.status(404).json({ message: "Movie not found" });
  res.json(movie);
});

app.get("/movies/:movieId/shows", (req: Request, res: Response) => {
  const id = parseInt(req.params.movieId as string);
  const movie = movies.find((m) => m.id === id);
  if (!movie) return res.status(404).json({ message: "Movie not found" });
  res.json({ shows: movie.shows });
});

app.post("/bookings/:userId", (req: Request, res: Response) => {
  const userId = parseInt(req.params.userId as string);
  const { movieId, showId, seats } = req.body || {};
  const user = users.find((u) => u.id === userId);
  if (!user) return res.status(404).json({ message: "user not found" });
  const movie = movies.find((m) => m.id === movieId);
  if (!movie) return res.status(404).json({ message: "Movie not found" });
  const show = movie.shows.find((s) => s.showId === showId);
  if (!show) return res.status(404).json({ message: "Show not found" });
  if (show.availableSeats < seats) {
    return res.status(400).json({ message: "Not enough seats available" });
  }
  show.availableSeats -= seats;
  const bookingId = GLOBAL_BOOKING_ID++;
  const totalAmount = show.pricePerSeat * seats;
  user.bookings.push({
    bookingId,
    movieId,
    showId,
    seats,
    totalAmount,
    status: "confirmed",
  });
  res.status(201).json({
    message: "Booking successful",
    bookingId,
    movieTitle: movie.title,
    showTime: show.time,
    seats,
    totalAmount,
  });
});

app.get("/bookings/:userId", (req: Request, res: Response) => {
  const userId = parseInt(req.params.userId as string);
  const user = users.find((u) => u.id === userId);
  if (!user) return res.status(404).json({ message: "user not found" });
  res.json({ bookings: user.bookings });
});

app.get("/bookings/:userId/:bookingId", (req: Request, res: Response) => {
  const userId = parseInt(req.params.userId as string);
  const bookingId = parseInt(req.params.bookingId as string);
  const user = users.find((u) => u.id === userId);
  const booking = user?.bookings.find((b) => b.bookingId === bookingId);
  if (!booking) return res.status(404).json({ message: "Booking not found" });
  res.json(booking);
});

app.put("/bookings/:userId/:bookingId", (req: Request, res: Response) => {
  const userId = parseInt(req.params.userId as string);
  const bookingId = parseInt(req.params.bookingId as string);
  const { seats } = req.body || {};
  const user = users.find((u) => u.id === userId);
  const booking = user?.bookings.find((b) => b.bookingId === bookingId);
  if (!booking) return res.status(404).json({ message: "Booking not found" });
  const movie = movies.find((m) => m.id === booking.movieId);
  const show = movie?.shows.find((s) => s.showId === booking.showId);
  if (!show) return res.status(404).json({ message: "Show not found" });
  if (show.availableSeats < seats) {
    return res.status(400).json({ message: "not enough seats" });
  }
  show.availableSeats -= seats;
  booking.seats += seats;
  booking.totalAmount = booking.seats * show.pricePerSeat;
  res.json({
    message: "Booking updated successfully",
    bookingId: booking.bookingId,
    seats: booking.seats,
    totalAmount: booking.totalAmount,
  });
});

app.delete("/bookings/:userId/:bookingId", (req: Request, res: Response) => {
  const userId = parseInt(req.params.userId as string);
  const bookingId = parseInt(req.params.bookingId as string);
  const user = users.find((u) => u.id === userId);
  const booking = user?.bookings.find((b) => b.bookingId === bookingId);
  if (!booking) return res.status(404).json({ message: "Booking not found" });
  booking.status = "cancelled";
  res.json({ message: "Booking cancelled successfully" });
});

app.get("/summary/:userId", (req: Request, res: Response) => {
  const userId = parseInt(req.params.userId as string);
  const user = users.find((u) => u.id === userId);
  if (!user) return res.status(404).json({ message: "user not found" });
  let totalAmountSpent = 0;
  let confirmedBookings = 0;
  let cancelledBookings = 0;
  let totalSeatsBooked = 0;
  for (const b of user.bookings) {
    totalAmountSpent += b.totalAmount;
    totalSeatsBooked += b.seats;
    if (b.status === "confirmed") confirmedBookings++;
    else if (b.status === "cancelled") cancelledBookings++;
  }
  res.json({
    userId,
    username: user.username,
    totalBookings: user.bookings.length,
    totalAmountSpent,
    confirmedBookings,
    cancelledBookings,
    totalSeatsBooked,
  });
});

app.listen(3000, () => {
  console.log("server running on port 3000");
});
