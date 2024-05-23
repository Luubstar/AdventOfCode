module Dia6 where 

-- ##############################################
-- ## Imports                                  ##
-- ##############################################
 



-- ##############################################
-- ## Definiciones de datos, tipos y clases    ##
-- ############################################## 

   type Tiempo = Int 
   type Distancia = Int 
   type Carrera = (Tiempo,Distancia)

-- ##############################################
-- ## Función main                             ##
-- ##############################################

   main = do
     contents <- (readFile "Input6.txt")
     let lin = lines contents
     putStrLn $ show $ part1 $ parseCarrera (tail $ words $ head lin) (tail $ words $ last lin)
     putStrLn "--------"
     putStrLn $ show $ part2 $ head $ parseCarrera [concat $ tail $ words $ head lin] [concat $ tail $ words $ last lin]

-- ##############################################
-- ## Funciones de la 1º parte                 ##
-- ##############################################

   part1 :: [Carrera] -> Int
   part1 lista = product (map getWins lista) 
-- ##############################################
-- ## Funciones de la 2º parte                 ##
-- ##############################################

   part2 :: Carrera -> Int 
   part2 carrera = getWins carrera

-- ##############################################
-- ## Funciones generales                      ##
-- ##############################################
   
   getWins :: Carrera -> Int
   getWins (t,d) = 
     let inicio = ceiling $ (fromIntegral(t) - sqrt(fromIntegral $ (t^2) - (4*d)) + 0.25)/2
         fin    = floor $ (fromIntegral(t) + sqrt(fromIntegral $ (t^2) - (4*d)) - 0.25)/2
     in (fin - inicio) + 1

   parseCarrera :: [String] -> [String] -> [Carrera]
   parseCarrera tiempos distancias = zip (map read tiempos) (map read distancias) 
