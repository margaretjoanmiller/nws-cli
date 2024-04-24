(in-package :nws-cli)

(defun get-office-grid (lat long)
  (com.inuoe.jzon:parse (flexi-streams:octets-to-string (dex:get (format "https://api.weather.gov/points/~d,~d" lat long)))))


(defun get-weather-ht 
    (flexi-streams:octets-to-string (dex:get "https://api.weather.gov/gridpoints/SGX/37,69/forecast"))
  )
