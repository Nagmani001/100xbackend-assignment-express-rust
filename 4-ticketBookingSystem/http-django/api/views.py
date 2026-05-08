from typing import TypedDict
from django.http import JsonResponse
from django.views.decorators.csrf import csrf_exempt
from django.views.decorators.http import require_http_methods, require_GET, require_POST
import json
import re


class Show(TypedDict):
    showId:int
    time:str
    pricePerSeat:int
    availableSeats:int

class Movie(TypedDict):
    id:int
    title:str
    genre:str
    duration:int
    shows:list[Show]

class Booking(TypedDict):
    bookingId:int
    movieId:int
    showId:int
    seats:int
    totalAmount:int
    status:str

class User(TypedDict):
    id:int
    username:str
    email:str
    password:str
    bookings:list[Booking]


users:list[User] = []
global_user_id = 1
global_booking_id = 1001

movies:list[Movie] = [
    {
        "id":1,
        "title":"Inception",
        "genre":"Sci-Fi",
        "duration":148,
        "shows":[
            {"showId":101,"time":"10:00 AM","pricePerSeat":200,"availableSeats":50},
            {"showId":102,"time":"2:00 PM","pricePerSeat":250,"availableSeats":50},
            {"showId":103,"time":"6:00 PM","pricePerSeat":300,"availableSeats":50},
        ],
    },
    {
        "id":2,
        "title":"The Dark Knight",
        "genre":"Action",
        "duration":152,
        "shows":[
            {"showId":201,"time":"11:00 AM","pricePerSeat":200,"availableSeats":50},
            {"showId":202,"time":"3:00 PM","pricePerSeat":250,"availableSeats":50},
            {"showId":203,"time":"7:00 PM","pricePerSeat":300,"availableSeats":50},
        ],
    },
    {
        "id":3,
        "title":"Interstellar",
        "genre":"Sci-Fi",
        "duration":169,
        "shows":[
            {"showId":301,"time":"12:00 PM","pricePerSeat":250,"availableSeats":50},
            {"showId":302,"time":"5:00 PM","pricePerSeat":300,"availableSeats":50},
        ],
    },
]

email_re = re.compile(r"^[^\s@]+@[^\s@]+\.[^\s@]+$")


def find_user_with_id(id:int):
    found_user = None
    for user in users:
        if(user["id"] == id):
            found_user = user
            break
    return found_user

def find_user_with_email(email:str):
    found_user = None
    for user in users:
        if(user["email"] == email):
            found_user = user
            break
    return found_user

def find_movie_with_id(id:int):
    found_movie = None
    for movie in movies:
        if(movie["id"] == id):
            found_movie = movie
            break
    return found_movie

def find_show_with_id(id:int,movie:Movie):
    found_show = None
    for show in movie["shows"]:
        if(show["showId"] == id):
            found_show = show
            break
    return found_show

def find_booking_with_id(id:int,user:User):
    found_booking = None
    for booking in user["bookings"]:
        if(booking["bookingId"] == id):
            found_booking = booking
            break
    return found_booking


def parse_body(request):
    try:
        return json.loads(request.body)
    except Exception:
        return None


@csrf_exempt
@require_POST
def signup(request):
    data = parse_body(request)
    if(data == None):
        return JsonResponse({ "message": "invalid input" },status=400)
    if(not isinstance(data.get("username"),str) or not isinstance(data.get("email"),str) or not isinstance(data.get("password"),str)):
        return JsonResponse({ "message": "invalid input" },status=400)
    if(not email_re.match(data["email"])):
        return JsonResponse({ "message": "invalid input" },status=400)
    user = find_user_with_email(data["email"])
    if not user == None:
        return JsonResponse({ "message": "user already exists" },status=401)
    global global_user_id
    uid = global_user_id
    users.append({
        "id":uid,
        "username":data["username"],
        "email":data["email"],
        "password":data["password"],
        "bookings":[],
    })
    global_user_id += 1
    return JsonResponse({ "message": "User created successfully", "userId": uid },status=201)


@require_GET
def get_movies(request):
    return JsonResponse({"movies":movies})


@require_GET
def get_movie_by_id(request,movieId):
    movie = find_movie_with_id(movieId)
    if movie == None:
        return JsonResponse({ "message": "Movie not found" },status=404)
    return JsonResponse(movie)


@require_GET
def get_movie_shows(request,movieId):
    movie = find_movie_with_id(movieId)
    if movie == None:
        return JsonResponse({ "message": "Movie not found" },status=404)
    return JsonResponse({"shows":movie["shows"]})


@csrf_exempt
def bookings_user(request,userId):
    if request.method == "POST":
        return post_booking(request,userId)
    if request.method == "GET":
        return get_user_bookings(request,userId)
    return JsonResponse({"error":"this method is not present bro, returning 405 as django would return "},status=405)


def post_booking(request,userId):
    data = parse_body(request)
    if data == None:
        return JsonResponse({ "message": "invalid input" },status=400)
    user = find_user_with_id(userId)
    if user == None:
        return JsonResponse({ "message": "user not found" },status=404)
    movie = find_movie_with_id(data.get("movieId"))
    if movie == None:
        return JsonResponse({ "message": "Movie not found" },status=404)
    show = find_show_with_id(data.get("showId"),movie)
    if show == None:
        return JsonResponse({ "message": "Show not found" },status=404)
    seats = data.get("seats",0)
    if show["availableSeats"] < seats:
        return JsonResponse({ "message": "Not enough seats available" },status=400)
    show["availableSeats"] -= seats
    global global_booking_id
    bid = global_booking_id
    global_booking_id += 1
    total_amount = show["pricePerSeat"] * seats
    user["bookings"].append({
        "bookingId":bid,
        "movieId":data["movieId"],
        "showId":data["showId"],
        "seats":seats,
        "totalAmount":total_amount,
        "status":"confirmed",
    })
    return JsonResponse({
        "message":"Booking successful",
        "bookingId":bid,
        "movieTitle":movie["title"],
        "showTime":show["time"],
        "seats":seats,
        "totalAmount":total_amount,
    },status=201)


def get_user_bookings(request,userId):
    user = find_user_with_id(userId)
    if user == None:
        return JsonResponse({ "message": "user not found" },status=404)
    return JsonResponse({"bookings":user["bookings"]})


@csrf_exempt
def manage_booking(request,userId,bookingId):
    if request.method == "GET":
        return get_specific_booking(request,userId,bookingId)
    if request.method == "PUT":
        return update_booking(request,userId,bookingId)
    if request.method == "DELETE":
        return delete_booking(request,userId,bookingId)
    return JsonResponse({"error":"this method is not present bro, returning 405 as django would return "},status=405)


def get_specific_booking(request,userId,bookingId):
    user = find_user_with_id(userId)
    if user == None:
        return JsonResponse({ "message": "Booking not found" },status=404)
    booking = find_booking_with_id(bookingId,user)
    if booking == None:
        return JsonResponse({ "message": "Booking not found" },status=404)
    return JsonResponse(booking)


def update_booking(request,userId,bookingId):
    data = parse_body(request)
    if data == None:
        return JsonResponse({ "message": "invalid input" },status=400)
    user = find_user_with_id(userId)
    if user == None:
        return JsonResponse({ "message": "Booking not found" },status=404)
    booking = find_booking_with_id(bookingId,user)
    if booking == None:
        return JsonResponse({ "message": "Booking not found" },status=404)
    movie = find_movie_with_id(booking["movieId"])
    if movie == None:
        return JsonResponse({ "message": "Movie not found" },status=404)
    show = find_show_with_id(booking["showId"],movie)
    if show == None:
        return JsonResponse({ "message": "Show not found" },status=404)
    seats = data.get("seats",0)
    if show["availableSeats"] < seats:
        return JsonResponse({ "message": "not enough seats" },status=400)
    show["availableSeats"] -= seats
    booking["seats"] += seats
    booking["totalAmount"] = booking["seats"] * show["pricePerSeat"]
    return JsonResponse({
        "message":"Booking updated successfully",
        "bookingId":booking["bookingId"],
        "seats":booking["seats"],
        "totalAmount":booking["totalAmount"],
    })


def delete_booking(request,userId,bookingId):
    user = find_user_with_id(userId)
    if user == None:
        return JsonResponse({ "message": "Booking not found" },status=404)
    booking = find_booking_with_id(bookingId,user)
    if booking == None:
        return JsonResponse({ "message": "Booking not found" },status=404)
    booking["status"] = "cancelled"
    return JsonResponse({ "message": "Booking cancelled successfully" })


@require_GET
def get_summary(request,userId):
    user = find_user_with_id(userId)
    if user == None:
        return JsonResponse({ "message": "user not found" },status=404)
    total_amount = 0
    confirmed = 0
    cancelled = 0
    total_seats = 0
    for booking in user["bookings"]:
        total_amount += booking["totalAmount"]
        total_seats += booking["seats"]
        if booking["status"] == "confirmed":
            confirmed += 1
        if booking["status"] == "cancelled":
            cancelled += 1
    return JsonResponse({
        "userId":userId,
        "username":user["username"],
        "totalBookings":len(user["bookings"]),
        "totalAmountSpent":total_amount,
        "confirmedBookings":confirmed,
        "cancelledBookings":cancelled,
        "totalSeatsBooked":total_seats,
    })
