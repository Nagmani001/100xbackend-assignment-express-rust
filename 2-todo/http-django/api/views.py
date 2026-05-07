from django.http import HttpResponse, JsonResponse
from django.views.decorators.csrf import csrf_exempt
from django.views.decorators.http import require_http_methods, require_GET, require_POST 
from django.shortcuts import render

todos = []
global_id = 1

@require_GET
def main(request):
    return HttpResponse(f"Welcome to TODO Backend!".encode())

@csrf_exempt
@require_POST
def add_todo(request):
    global global_id
    task = request.GET.get("task","")
    todos.append({"id":global_id, "task":task,"completed":False})
    global_id += 1
    return JsonResponse({
        "message":"TODO added successfully!" ,
        "todoCount":len(todos)
    },status=201)


@require_GET
def get_todos(request):
    if(len(todos) == 0):
        return JsonResponse({ "message": "No TODOs found yet." },status=404)
    else:
        return JsonResponse(todos,safe=False)

@require_GET
def todo_by_id(request,id):
    for todo in todos:
        if(todo["id"] == id):
            return JsonResponse(todo,safe=False)
    return JsonResponse({ "error": "TODO not found" },status=404)



@csrf_exempt
@require_http_methods(["PUT"])
def update_todo(request,id):
    for todo in todos:
        if(todo["id"] == id):
            todo["completed"] = True
            return JsonResponse({ "message": "TODO marked as completed!", "todo":todo},safe=False)
    return JsonResponse({ "error": "TODO not found" },status=404)


@csrf_exempt
@require_http_methods(["DELETE"])
def delete_todo(request, id):
    found = None
    global todos
    for todo in todos:
        if(todo["id"] == id):
            found = todo
    if found == None:
        return JsonResponse({ "error": "TODO not found" },status=404)
   # too hard syntax , probably should make this a little verbose 
    todos = [t for t in todos if t["id"] != id]
    return JsonResponse({ "message": "TODO deleted successfully!" })

def filter_todo(request):
    status = request.GET.get("status","")
    if(status == "completed"):
        completed = []
        for todo in todos:
            if(todo["completed"] == True):
                completed.append(todo)
        return JsonResponse(completed,safe=False)
    if(status == "pending"):
        pending= []
        for todo in todos:
            if(todo["completed"] == False):
                pending.append(todo)
        return JsonResponse(pending,safe=False)
    return JsonResponse({ "message": "No TODOs found for the given filter." },status=404)




