from django.urls import path
from api import views

urlpatterns = [
    path('signup', views.signup),
    path('users',views.get_users),
    path('bookings/<int:userId>',views.bookings_user),
    path('bookings/<int:userId>/<int:bookingId>',views.manage_bookings),
    path('bookings/<int:userId>/<int:bookingId>/status',views.update_status),
    path('summary/<int:userId>',views.get_summary),


]
