import os
import sys
from django.shortcuts import render
from django.http import HttpResponse, JsonResponse


BASE_DIR = os.path.dirname(os.path.abspath(__file__))
NOTES_FILE = os.path.join(BASE_DIR, "notes.txt")

users = []
blocked_users = {}

def main(request):
    return HttpResponse(f"Welcome to my first Express or Actix server!".encode())

def greet(request):
    name = request.GET.get("name","")
    return HttpResponse(f"Hello {name}, nice to meet you!".encode())

def write(request):
    msg = request.GET.get("message","")
    with open(NOTES_FILE, "w") as f:
        f.write(msg)
    return HttpResponse(f"wrote to file".encode())

def append(request):
    msg = request.GET.get("message","")
    with open(NOTES_FILE, "a") as f:
        f.write(msg)
    return HttpResponse(f"appended to file".encode())

def read(request):
    try:
        with open(NOTES_FILE, "rb") as f:
            data = f.read()
    except FileNotFoundError:
        return HttpResponse(f"No notes found yet.".encode())
    return HttpResponse(data,content_type="text/plain; charset=utf-8") 

def clear(request):
    with open(NOTES_FILE, "w") as f:
        f.write("")
    return HttpResponse(f"cleared".encode()) 

def add_user(request):
    name = request.GET.get("name","")
    age = request.GET.get("age","")
    users.append({"name":name,"age":int(age)})
    return HttpResponse(f"User added successfully!".encode())

def check_user(request):
    name = request.GET.get("name","")
    age = request.GET.get("age","0")
    if(int(age) < 18):
        blocked_users[name] = True
        return HttpResponse(f"You are blocked as your age is less.".encode())
    else:
        blocked_users[name] = False
        return HttpResponse(f"Access granted!".encode())

def is_blocked(request):
    name = request.GET.get("name","")
    if(blocked_users[name]):
        return HttpResponse(f"{name} is blocked.".encode())
    else:
        return HttpResponse(f"{name} is not blocked.".encode())

def list_users(request):
    return JsonResponse(users,safe=False)


def clear_data(request):
    users.clear()
    blocked_users.clear()
    return HttpResponse(f"All data cleared successfully!".encode())

