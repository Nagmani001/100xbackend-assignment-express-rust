from django.urls import path

from api import views


urlpatterns = [
    path('', views.main),
    path('add-todo', views.add_todo),
    path('todos', views.get_todos),
    path('todo/<int:id>', views.todo_by_id),
    path('todos/<int:id>/complete', views.update_todo),
    path('todos/<int:id>', views.delete_todo),
    path('todos/filter', views.filter_todo),
]
