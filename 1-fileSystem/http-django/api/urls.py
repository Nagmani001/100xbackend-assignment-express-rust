from django.urls import path
from api import views

urlpatterns = [
    path('', views.main),
    path('greet', views.greet),
    path('write', views.write),
    path('append', views.append),
    path('read', views.read),
    path('clear', views.clear),
    path('add-user', views.add_user),
    path('check-user', views.check_user),
    path('is-blocked', views.is_blocked),
    path('users', views.list_users),
    path('clear-data', views.clear_data),
]
