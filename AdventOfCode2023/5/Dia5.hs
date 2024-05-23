module Dia5 where
  import Debug.Trace (trace) 
  import Data.List.Split (splitOn) -- ¡¡IMPORTANTE!! Usa cabal install split en tu consola para poder ejecutar este import 
  import Data.List (find,sort)

  type Informacion = (Int,Int,Int)

  ---- PARTE 1 ----------------------------------------------------------------

  -- Crea una lista de los resultados de las semillas
  part1 :: [[Informacion]] -> [Int] -> [Int]
  part1 info (seed:list) = [getInt info seed] ++ (part1 info list)
  part1 info [] =  []

  -- Obtiene el valor de una semilla a través de todas sus instrucciones
  getInt :: [[Informacion]] -> Int -> Int 
  getInt (first:list) seed = getInt list (getNewValue seed first )
  getInt [] seed = seed

  -- Obtiene el valor de una semilla respecto a un set de instrucciones de forma recursiva
  getNewValue :: Int -> [Informacion] -> Int
  getNewValue seed ((fin, inicio, rango):lista)
   | contenida seed = seed + (fin - inicio) 
   | otherwise = getNewValue seed lista
   where 
    -- Minifunción para revisar si la semilla está contenida en el rango de la instrucción
    contenida i = i >= inicio && i <= (inicio + rango)
  getNewValue s [] = s

  ---- PARTE 2 ---------------------------------------------------------------
  
  -- Obtiene el valor del menor de todos los rangos de la semilla dadas las instrucciones
  -- Hay mejores formas de sacar el valor mínimo. Me da pereza refactorizar esta basura
  part2 :: [[Informacion]] -> [Int] -> Int -> Int
  part2 _ [] menor = menor
  part2 info (seed:range:list) menor =
    let v = getInt' info [(seed,(seed + (range-1)))]
    in if (menor == (-1) || v < menor) then (part2 info list v) else (part2 info list menor)

  -- Obtiene el menor valor dado un rango de semillas tras ejecturas las instrucciones
  getInt' :: [[Informacion]]  -> [(Int,Int)] -> Int
  getInt' (primera:lista) rangos = getInt' lista (getValueInRange primera primera rangos)
  getInt' [] rangos = getMin rangos (-1) 
  
{-Sé que parece que solo Dios y yo sabemos qué hace esta función, pero lo explicaré
  Lee dos listas de instrucciones y una lista de rangos. El 2º set de instrucciones
  actua de guardado, dado que esto es un doble blucle for (para instrucciones y para rangos)
  
  Por cada rango en la entrada, se revisa:
  -> Es contenido en una instrucción -> Se avanza el rango y se pasa al siguiente
  -> Empieza en el rango pero se sale fuera, o empieza fuera y entra en el rango ->
  La parte en el rango se saca de la función y se avanza, el resto vuelve a iterar en la función
  
  Por tanto, devuelve una lista de rangos dado dos listas de instrucciones idénticas-}
  getValueInRange :: [Informacion] -> [Informacion]-> [(Int,Int)] -> [(Int,Int)]
  getValueInRange ((fin,inicio, rango):l) save ((i,f):lista) 
   | contenido i && contenido f       =  [(i+paso, f+paso)] ++ (getValueInRange save save lista)
   | contenido i && not (contenido f) =  [(i+paso, (inicio+rango) + paso)] ++ (getValueInRange save save (((inicio+rango+1), f):lista))
   | not(contenido i) && contenido f  = [(inicio+paso, f+paso)] ++ (getValueInRange save save ((i, inicio-1):lista)) 
   | otherwise                        = (getValueInRange l save ((i,f):lista))
   where 
    -- Minifunción para ver si está contenido v en el rango
    contenido v = v>=inicio && v <= (inicio + rango)
    paso = (fin - inicio)
  getValueInRange [] s (e:l)= [e] ++ getValueInRange s s l
  getValueInRange _ _ [] =[]

  -- Función para obtener el valor mínimo de un rango. Seguro que hay mejores formas de hacerlo
  getMin :: [(Int,Int)] -> Int -> Int
  getMin ((x,y):xs) m = 
    let v = minimum [x,y]
    in if (v < m || m == -1) then getMin xs v else getMin xs m
  getMin [] m = m


  -- Traduce la entrada a una lista de enteros de formato inicio fin rango---
  
  -- Pasa por la entrada creando la listas de instrucciones
  cleanDatabase :: [String] -> [[Informacion]]
  cleanDatabase [] = []
  cleanDatabase (line: list)
   | (length $ words line) == 2 = [(cleanline (line:list))] ++ cleanDatabase list 
   | otherwise = [] ++ cleanDatabase list
  
  -- Dada una cadena, una instrucción, la divide en una tripleta (x,y,z)
  cleanline :: [String] -> [Informacion]
  cleanline (line:list) 
   | line == "" = []
   | (length $ words line) == 2 = [] ++ cleanline(list)
   | otherwise = 
       let x = read $ (words line)!!0
           y = read $ (words line)!!1
           z = read $ (words line)!!2
      in [(x,y,z)] ++ cleanline (list)
  cleanline [] = []

 -- Fin del parser ---------------------------------------------------------

 -- Inicia ejecución --
  main = do 
   contents <- (readFile "Input5.txt")
   let lin = lines contents
   putStrLn $ show $ sort $ minimum $  part1 (cleanDatabase $ drop 2 lin) (map read $ tail $ words $ head lin) 
   putStrLn "--------"
   putStrLn $ show $ part2 (cleanDatabase $ drop 2 lin) (map read $ tail $ words $ head lin) (-1)
