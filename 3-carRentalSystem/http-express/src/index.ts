import express, { Request, Response } from "express";

const app = express();

app.use(express.json());
let GLOBAL_ID = 1;
let GLOBAL_BOOKING_ID = 101;


interface Booking {
  bookingId: number,
  carName: string,
  days: number,
  rentPerDay: number,
  status: string,
  totalCost: number
}
interface User {
  id: number,
  username: string,
  password: string,
  bookings: Booking[]
}

let users: User[] = [];

app.post("/signup", (req: Request, res: Response) => {
  const { username, password } = req.body;

  users.push({
    id: GLOBAL_ID,
    username,
    password,
    bookings: [],
  });
  GLOBAL_ID++;

  res.status(201).json({ message: "User created successfully", userId: 1 });
});

app.get("/users", (req: Request, res: Response) => {
  res.json(users);
});

app.post("/bookings/:userId", (req: Request, res: Response) => {
  console.log(users);
  const { carName, days, rentPerDay } = req.body;
  const userId = req.params.userId as string;

  const user = users.find(x => x.id == parseInt(userId));

  let bookingId = GLOBAL_BOOKING_ID;
  user?.bookings.push({
    carName,
    bookingId,
    days,
    rentPerDay,
    status: "booked",
    totalCost: days * rentPerDay
  },);
  GLOBAL_BOOKING_ID++;

  console.log(users);
  res.status(201).json({
    message: "booking complete",
    bookingId,
    totalCost: days * rentPerDay
  });
});


app.get("/bookings/:userId", (req: Request, res: Response) => {
  const userId = req.params.userId as string;
  const bookings = users.find(x => x.id == parseInt(userId))?.bookings;
  res.json(bookings);
});

app.get("/bookings/:userId/:bookingId", (req: Request, res: Response) => {
  const userId = req.params.userId as string;
  const bookingId = req.params.bookingId as string;

  const booking = users.find(x => x.id == parseInt(userId))?.bookings.find(x => x.bookingId == parseInt(bookingId));
  if (!booking) {
    res.status(404).json({ message: "Booking not found" });
  }
  res.json(booking);
});

app.put("/bookings/:userId/:bookingId", (req: Request, res: Response) => {
  console.log(users);
  const userId = req.params.userId as string;
  const bookingId = req.params.bookingId as string;
  const { carName, days, rentPerDay } = req.body;

  const booking = users.find(x => x.id == parseInt(userId))?.bookings.find(x => x.bookingId == parseInt(bookingId));
  if (!booking) return;
  if (carName) {
    booking.carName = carName;
  }
  if (days) {
    booking.days = days;
  }
  if (carName) {
    booking.rentPerDay = rentPerDay;
  }
  if (rentPerDay && days) {
    booking.totalCost = days * rentPerDay
  } else if (rentPerDay) {
    booking.totalCost = rentPerDay * booking.days;

  } else if (days) {
    booking.totalCost = days * booking.rentPerDay;
  }
  console.log(users);
  res.json(booking);
});

app.put("PUT /bookings/:userId/:bookingId/status", (req: Request, res: Response) => {

  const userId = req.params.userId as string;
  const bookingId = req.params.bookingId as string;

  const booking = users.find(x => x.id == parseInt(userId))?.bookings.find(x => x.bookingId == parseInt(bookingId));
  if (!booking) return;

  booking.status = booking.status == "booked" ? "completed" : "cancelled";
  res.json({ message: "Status updated successfully" });
});

app.delete("/bookings/:userId/:bookingId", (req, res) => {

  console.log(users);
  const userId = req.params.userId as string;
  const bookingId = req.params.bookingId as string;

  const bookings = users.find(x => x.id == parseInt(userId))?.bookings;

  if (!bookings) return;

  bookings.filter(x => x.bookingId !== parseInt(bookingId));

  res.json({ message: "Booking deleted successfully" });
});


app.get("/summary/:userId", (req: Request, res: Response) => {
  const userId = req.params.userId as string;
  const user = users.find(x => x.id == parseInt(userId));
  let totalSpent = 0;
  user?.bookings.forEach(x => {
    totalSpent += x.totalCost;
  });

  res.json({
    userId,
    username: user?.username,
    totalBookings: user?.bookings.length,
    totalAmountSpent: totalSpent
  });
});

app.listen(3000, () => {
  console.log("Server running on port 3000");
});
