import express, { Request, Response } from "express";

const app = express();
app.use(express.json());

let GLOBAL_ID = 1;
let GLOBAL_BOOKING_ID = 101;

interface Booking {
  bookingId: number;
  carName: string;
  days: number;
  rentPerDay: number;
  status: string;
  totalCost: number;
}

interface User {
  id: number;
  username: string;
  password: string;
  bookings: Booking[];
}

let users: User[] = [];

const bookingPublic = (b: Booking) => ({
  bookingId: b.bookingId,
  carName: b.carName,
  days: b.days,
  rentPerDay: b.rentPerDay,
  status: b.status,
});

app.post("/signup", (req: Request, res: Response) => {
  const { username, password } = req.body || {};
  if (typeof username !== "string" || typeof password !== "string") {
    return res.status(400).json({ message: "invalid data" });
  }
  if (users.find((u) => u.username === username)) {
    return res.status(401).json({ message: "user already exist" });
  }
  const id = GLOBAL_ID++;
  users.push({ id, username, password, bookings: [] });
  res.status(201).json({ message: "User created successfully", userId: id });
});

app.get("/users", (_req: Request, res: Response) => {
  res.json({ users });
});

app.post("/bookings/:userId", (req: Request, res: Response) => {
  const { carName, days, rentPerDay } = req.body || {};
  const userId = parseInt(req.params.userId as string);
  const user = users.find((u) => u.id === userId);
  if (!user) return res.status(404).json({ message: "user not found" });

  const bookingId = GLOBAL_BOOKING_ID++;
  const totalCost = days * rentPerDay;
  user.bookings.push({
    bookingId,
    carName,
    days,
    rentPerDay,
    status: "booked",
    totalCost,
  });
  res.status(201).json({ message: `${carName} booked`, bookingId, totalCost });
});

app.get("/bookings/:userId", (req: Request, res: Response) => {
  const userId = parseInt(req.params.userId as string);
  const user = users.find((u) => u.id === userId);
  if (!user) return res.status(404).json({ message: "user not found" });
  res.json({ bookings: user.bookings.map(bookingPublic) });
});

app.get("/bookings/:userId/:bookingId", (req: Request, res: Response) => {
  const userId = parseInt(req.params.userId as string);
  const bookingId = parseInt(req.params.bookingId as string);
  const user = users.find((u) => u.id === userId);
  const booking = user?.bookings.find((b) => b.bookingId === bookingId);
  if (!booking) return res.status(404).json({ message: "booking not found" });
  res.json(bookingPublic(booking));
});

app.put("/bookings/:userId/:bookingId/status", (req: Request, res: Response) => {
  const userId = parseInt(req.params.userId as string);
  const bookingId = parseInt(req.params.bookingId as string);
  const { status } = req.body || {};
  const user = users.find((u) => u.id === userId);
  const booking = user?.bookings.find((b) => b.bookingId === bookingId);
  if (!booking) return res.status(404).json({ message: "booking not found" });
  booking.status = status;
  res.json({ message: "Status updated successfully" });
});

app.put("/bookings/:userId/:bookingId", (req: Request, res: Response) => {
  const userId = parseInt(req.params.userId as string);
  const bookingId = parseInt(req.params.bookingId as string);
  const { carName, days, rentPerDay } = req.body || {};
  const user = users.find((u) => u.id === userId);
  const booking = user?.bookings.find((b) => b.bookingId === bookingId);
  if (!booking) return res.status(404).json({ message: "booking not found" });
  if (carName) booking.carName = carName;
  if (typeof days === "number") booking.days = days;
  if (typeof rentPerDay === "number") booking.rentPerDay = rentPerDay;
  booking.totalCost = booking.days * booking.rentPerDay;
  res.json(bookingPublic(booking));
});

app.delete("/bookings/:userId/:bookingId", (req: Request, res: Response) => {
  const userId = parseInt(req.params.userId as string);
  const bookingId = parseInt(req.params.bookingId as string);
  const user = users.find((u) => u.id === userId);
  if (!user) return res.status(404).json({ message: "user not found" });
  const before = user.bookings.length;
  user.bookings = user.bookings.filter((b) => b.bookingId !== bookingId);
  if (user.bookings.length === before) {
    return res.status(404).json({ message: "booking not found" });
  }
  res.json({ message: "Booking deleted successfully" });
});

app.get("/summary/:userId", (req: Request, res: Response) => {
  const userId = parseInt(req.params.userId as string);
  const user = users.find((u) => u.id === userId);
  if (!user) return res.status(404).json({ message: "user not found" });
  const totalAmountSpent = user.bookings.reduce((s, b) => s + b.totalCost, 0);
  res.json({
    userId,
    username: user.username,
    totalBookings: user.bookings.length,
    totalAmountSpent,
  });
});

app.listen(3000, () => {
  console.log("Server running on port 3000");
});
