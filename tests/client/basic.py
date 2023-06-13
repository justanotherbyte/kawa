import kawa
import requests
import timeit

def kawa_request():
    response = kawa.request("GET", "127.0.0.1", 8080, "/")
    print(response)

def requests_request():
    response = requests.request("GET", "http://127.0.0.1:8080/")
    print(response)

kawa_time = timeit.timeit(kawa_request, number=500)
requests_time = timeit.timeit(requests_request, number=500)

print("Kawa:", kawa_time)
print("Requests: ", requests_time)

print("Kawa is {}x faster".format(round(requests_time / kawa_time)))