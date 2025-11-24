import express, { Request, Response } from "express";
import fs from "fs";

const app = express();


app.use(express.json());

let userId = 1;
let BOOKINGID = 1001;

app.post("/signup", (req: Request, res: Response) => {
  const { username, password, email } = req.body;
  const id = userId++;

  const user = {
    id: id,
    username,
    password,
    email,
    bookings: []
  };
  const data = fs.readFileSync("./data.txt", "utf-8");

  const parsedData = JSON.parse(data);
  parsedData.users.push(user);

  fs.writeFileSync("./data.txt", JSON.stringify(parsedData));

  res.status(201).json({
    message: "User created successfully",
    userId: id
  })
});


app.get("/movies", (req: Request, res: Response) => {
  const data = fs.readFileSync("./data.txt", "utf-8");
  const parsedData = JSON.parse(data);
  const movies = parsedData.movies;

  res.status(200).json(movies);
});


app.get("/movies/:movieId", (req: Request, res: Response) => {
  const id = req.params.movieId as string;

  const data = fs.readFileSync("./data.txt", "utf-8");
  const parsedData = JSON.parse(data);
  const movie = parsedData.movies.find((x: any) => x.id == parseInt(id));

  if (!movie) {
    res.status(404).json({ message: "Movie not found" });
  } else {
    res.json(movie);
  }
});

app.get("/movies/:movieId/shows", (req: Request, res: Response) => {
  const id = req.params.movieId as string;

  const data = fs.readFileSync("./data.txt", "utf-8");
  const parsedData = JSON.parse(data);
  const movie = parsedData.movies.find((x: any) => x.id == parseInt(id));
  res.json(movie.shows);
});

app.post("/bookings/:userId", (req: Request, res: Response) => {
  const userId = req.params.userId;
  const { movieId, showId, seats } = req.body;
  const data = fs.readFileSync("./data.txt", "utf-8");
  const parsedData = JSON.parse(data);

  const movie = parsedData.movies.find((x: any) => x.id == parseInt(movieId));
  const show = movie.shows.find((x: any) => x.showId == parseInt(showId));

  if (show.availableSeats < seats) {
    return res.json({ message: "Not enough seats available" });
  }

  const bookingId = BOOKINGID++;
  const booking = {
    bookingId: bookingId,
    movieId,
    showId,
    seats,
    totalAmount: show.pricePerSeat * parseInt(seats),
    status: "confirmed",
    bookingDate: new Date()
  };


  const user = parsedData.users.find((x: any) => x.id == userId);
  user.bookings.push(booking);
  console.log(parsedData);

  fs.writeFileSync("./data.txt", JSON.stringify(parsedData));


  res.status(201).json({
    message: "Booking successful",
    bookingId: bookingId,
    movieTitle: movie.title,
    showTime: show.time,
    seats,
    totalAmount: show.pricePerSeat * parseInt(seats)
  });
});


app.get("/bookings/:userId", (req: Request, res: Response) => {
  const userId = req.params.userId as string;
  const data = fs.readFileSync("./data.txt", "utf-8");
  const parsedData = JSON.parse(data);
  const user = parsedData.users.find((x: any) => x.id == parseInt(userId));
  res.status(200).json(user.bookings);
});

app.get("/bookings/:userId/:bookingId", (req: Request, res: Response) => {
  const userId = req.params.userId as string;
  const bookingId = req.params.bookingId as string;

  const data = fs.readFileSync("./data.txt", "utf-8");
  const parsedData = JSON.parse(data);
  const user = parsedData.users.find((x: any) => x.id == parseInt(userId));
  const booking = user.bookings.find((x: any) => x.bookingId == parseInt(bookingId));

  if (!booking) {
    res.status(404).json({ "message": "Booking not found" })
  } else {
    res.json(booking);
  }
});


app.put("/bookings/:userId/:bookingId", (req: Request, res: Response) => {
  const userId = req.params.userId as string;
  const bookingId = req.params.bookingId as string;
  const { seats } = req.body;

  const data = fs.readFileSync("./data.txt", "utf-8");
  const parsedData = JSON.parse(data);
  const users = parsedData.users.find((x: any) => x.id == parseInt(userId));

  const booking = users.bookings.find((x: any) => x.bookingId == parseInt(bookingId));
  console.log("booking before", booking);

  const movieId = booking.movieId;
  const showId = booking.showId;

  const movie = parsedData.movies.find((x: any) => x.id == parseInt(movieId));
  const show = movie.shows.find((x: any) => x.showId == parseInt(showId));

  if (show.availableSeats < parseInt(seats)) {
    return res.status(404).json({
      message: "not enough seats"
    });
  }
  show.availableSeats -= parseInt(seats);
  const seat = parseInt(booking.seats) + parseInt(seats);
  booking.seats = seat.toString();
  booking.totalAmount = booking.seats * show.pricePerSeat;

  console.log("booking after", booking);


  res.json({
    message: "Booking updated successfully",
    bookingId: booking.bookingId,
    seats: booking.seats,
    totalAmount: booking.totalAmount,
  });

});

app.delete("/bookings/:userId/:bookingId", (req: Request, res: Response) => {

  const userId = req.params.userId as string;
  const bookingId = req.params.bookingId as string;
  const { seats } = req.body;

  const data = fs.readFileSync("./data.txt", "utf-8");
  const parsedData = JSON.parse(data);
  const users = parsedData.users.find((x: any) => x.id == parseInt(userId));

  const booking = users.bookings.find((x: any) => x.bookingId == parseInt(bookingId));
  booking.status = "cancelled";

  res.json({ message: "Booking cancelled successfully" });
});

app.get("/summary/:userId", (req: Request, res: Response) => {
  const userId = req.params.userId as string;

  const data = fs.readFileSync("./data.txt", "utf-8");
  const parsedData = JSON.parse(data);
  const user = parsedData.users.find((x: any) => x.id == userId);
  let totalAmount = 0;
  let totalConfirmedBooking = 0;
  let totalCancelledBooking = 0;
  let totalSeatsBooked = 0;

  user.bookings.forEach((x: any) => {
    totalAmount += x.totalAmount;
    if (x.status == "confirmed") {
      totalConfirmedBooking += 1;
    } else {
      totalCancelledBooking += 1;
    }
    totalSeatsBooked += x.seats;


  });


  res.json({
    userId: parseInt(userId),
    username: user.username,
    totalBookings: user.bookings.length,
    totalAmountSpent: totalAmount,
    confirmedBookings: totalConfirmedBooking,
    cancelledBookings: totalCancelledBooking,
    totalSeatsBooked: totalSeatsBooked
  });
});

app.listen(3000, () => {
  console.log("server running on port 3000");
});


