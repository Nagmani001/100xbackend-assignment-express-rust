from django.urls import path
from api import views

urlpatterns = [
    path('signup', views.signup),
    path('movies', views.get_movies),
    path('movies/<int:movieId>', views.get_movie_by_id),
    path('movies/<int:movieId>/shows', views.get_movie_shows),
    path('bookings/<int:userId>', views.bookings_user),
    path('bookings/<int:userId>/<int:bookingId>', views.manage_booking),
    path('summary/<int:userId>', views.get_summary),
]
