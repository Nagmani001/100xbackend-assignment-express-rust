from typing import TypedDict
from urllib import parse
from django.http import JsonResponse
from django.views.decorators.csrf import csrf_exempt
from django.shortcuts import HttpResponse, render
import json 
from django.views.decorators.http import require_http_methods, require_GET, require_POST 

class Booking(TypedDict):
    bookingId:int
    carName:str
    days:int
    rentPerDay:int
    status:str
    totalCost:int

class User(TypedDict):
    id:int
    username:str
    password:str
    bookings:list[Booking]
users:list[User] = []
global_id = 1
global_booking_id = 101

def find_user_with_username(username:str):
    found_user = None;
    for user in users:
        if(user["username"] == username):
            found_user = user
            break
    return found_user

def find_user_with_id(id:int):
    found_user = None;
    for user in users:
        if(user["id"] == id):
            found_user = user
            break
    return found_user

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
    if (data == None or not isinstance(data.get("username"),str) or not isinstance(data.get("password"),str)):
        return JsonResponse({ "message": "invalid data" },status=400)
    user = find_user_with_username(data["username"])
    if not user == None:
        return JsonResponse({ "message": "user already exist" },status=401)
    global global_id
    uid = global_id
    users.append({
        "id": uid,
        "username": data["username"],
        "password": data["password"],
        "bookings": [],
    })
    global_id +=1
    return JsonResponse({ "message": "User created successfully", "userId": uid },status=201)

def get_users(request):
    return JsonResponse({"users":users},safe=False)



@csrf_exempt
def bookings_user(request,userId):
    if request.method == "POST":
        return post_booking(request,userId)
    if request.method == "GET":
        return get_bookings(request,userId)
    return JsonResponse({"error":"this method is not present bro, returning 405 as django would return "},status=405)


@csrf_exempt
@require_POST
def post_booking(request,userId):
    data = parse_body(request)
    user = find_user_with_id(userId)
    if user == None or data == None:
        return JsonResponse({ "message": "user not found" },status=404)
    global global_booking_id
    bid = global_booking_id
    global_booking_id +=1 
    total_cost = data["days"] * data["rentPerDay"]
    carname = data["carName"]

    user["bookings"].append({
    "bookingId":bid,
    "carName":data["carName"],
    "days":data["days"],
    "rentPerDay":data["rentPerDay"],
    "status":"booked",
    "totalCost":total_cost,
    })
    return JsonResponse({ "message": f"{carname} booked", "bookingId":bid, "totalCost":total_cost},status=201)


@require_GET
def get_bookings(request,userId):
    user = find_user_with_id(userId)
    if user is None:
        return JsonResponse({ "message": "user not found" },status=404)
    bookings = []
    for booking in user["bookings"]:
        bookings.append({
           "bookingId":booking["bookingId"],
            "carName":booking["carName"],
            "days":booking["days"],
            "rentPerDay":booking["rentPerDay"],
            "status":booking["status"]
        })
    return JsonResponse({"bookings":bookings})

@csrf_exempt
def manage_bookings(request,userId,bookingId):
    if request.method == "GET":
        return get_specific_booking(request,userId,bookingId)
    if request.method == "DELETE":
        return delete_booking(request,userId,bookingId)
    return JsonResponse({"error":"this method is not present bro, returning 405 as django would return "},status=405)



@require_GET
def get_specific_booking(request,userId,bookingId):
    user = find_user_with_id(userId)
    if user == None:
        return JsonResponse({ "message": "booking not found" },status=404)
    booking = find_booking_with_id(bookingId,user)
    if booking == None:
        return JsonResponse({ "message": "booking not found" },status=404)
    return JsonResponse({
           "bookingId":booking["bookingId"],
            "carName":booking["carName"],
            "days":booking["days"],
            "rentPerDay":booking["rentPerDay"],
            "status":booking["status"]
    },safe=False)


@csrf_exempt
@require_http_methods(["PUT"])
def update_status(request,userId,bookingId):
    data = parse_body(request)
    user = find_user_with_id(userId)
    if user == None or data == None:
        return JsonResponse({ "message": "booking not found" },status=404)
    booking = find_booking_with_id(bookingId,user)
    if booking == None:
        return JsonResponse({ "message": "booking not found" },status=404)
    booking["status"] = data.get("status")
    return JsonResponse({ "message": "Status updated successfully" })

def delete_booking(request,userId,bookingId):
    user = find_user_with_id(userId)
    if user == None :
        return JsonResponse({ "message": "booking not found" },status=404)
    new_bookings = []
    for booking in user["bookings"]:
        if booking["bookingId"] != bookingId :
            new_bookings.append(booking)
    if(len(new_bookings) == len(user["bookings"])):
        return JsonResponse({ "message": "booking not found" },status=404)
    user["bookings"] = new_bookings
    return JsonResponse({ "message": "Booking deleted successfully" })


def get_summary(request,userId):
    user = find_user_with_id(userId)
    if user == None:
        return JsonResponse({ "message": "user not found" },status=404)
    total = 0 ; 
    for booking in user["bookings"]:
        total += booking["totalCost"]
    return JsonResponse({
        "userId":userId,
        "username":user["username"],
        "totalBookings":len(user["bookings"]),
        "totalAmountSpent":total
    })
