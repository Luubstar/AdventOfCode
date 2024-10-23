module Dia15 where 

-- ##############################################
-- ## Imports                                  ##
-- ##############################################

  import Data.List.Split (splitOn)
  import Data.List (elem, elemIndex, delete)
  import Data.Char (ord, digitToInt)
  import Debug.Trace (trace)
  import Data.Maybe (fromJust)

-- ##############################################
-- ## Definiciones de datos, tipos y clases    ##
-- ############################################## 

  type Label = String 
  type Lens = Int
  type Slot = (Label, Lens)
  type Box = [Slot]  

-- ##############################################
-- ## Función main                             ##
-- ##############################################

  main = do
   contents <- (readFile "Input15.txt")
   let lin = splitOn "," contents
       boxes = [[] | a <- [0..255]]

   putStrLn $ show $ sum $ map (part1 0) (lin)
   putStrLn "--------"
   putStrLn $ show $ part2 lin boxes 
  
-- ##############################################
-- ## Funciones de la 1º parte                 ##
-- ##############################################

  part1 :: Int -> String -> Int 
  part1 v (x:xs)  
   | x == '\n' = part1 v xs
   | otherwise = part1 (mod ((v + ord x) * 17) 256) xs
  part1 v [] = v 

-- ##############################################
-- ## Funciones de la 2º parte                 ##
-- ##############################################

  part2 :: [String] -> [Box] -> Int
  part2 (x:xs) l = 
    let cadena = fst $ dividirOperacion x 
        operacion = snd $ dividirOperacion x
    in part2 xs (editarCajas cadena operacion l)
  part2 [] l = toInt l 0 0

  editarCajas :: String -> String -> [Box] -> [Box]
  editarCajas cadena op cajas = 
   let indexcaja =  (part1 0 cadena)
       caja = cajas!!indexcaja
   in if (head op) == '-' 
    then 
      case buscarSlot cadena caja of 
       Just slot -> 
        let nuevacaja = delete slot caja 
        in actualizarLista cajas indexcaja nuevacaja
       Nothing -> cajas
    else 
       case buscarSlot cadena caja of 
        Just slot -> 
          let indiceSlot = fromJust (elemIndex slot caja)
              nuevacaja = actualizarLista caja indiceSlot (cadena, digitToInt $ last op)
              in actualizarLista cajas indexcaja nuevacaja
        Nothing   -> 
         let nuevacaja = caja ++ [(cadena, digitToInt $ last op)]
         in actualizarLista cajas indexcaja nuevacaja

  toInt :: [Box] -> Int -> Int -> Int 
  toInt (((label, lens):slots):cajas) pos boxpos = ((boxpos + 1 ) * (pos+1) * lens) + toInt ((slots):cajas) (pos+1) boxpos
  toInt ([]:cajas) _ boxpos = toInt cajas 0 (boxpos + 1)
  toInt [] _ _= 0

-- ##############################################
-- ## Funciones generales                      ##
-- ##############################################

  dividirOperacion :: String -> (String, String)
  dividirOperacion x 
   | elem '=' x =
     let index = fromJust (elemIndex '=' x)
         op = if (last $ drop index x) == '\n' then init $ drop index x else drop index x
     in (take index x, op)
   | elem '-' x = 
     let index = fromJust (elemIndex '-' x)
         op = if (last $ drop index x) == '\n' then init $ drop index x else drop index x
     in (take index x, op)

  buscarSlot :: String -> Box -> Maybe Slot
  buscarSlot c (x:xs)
   | c == (fst x) = Just x
   | otherwise = buscarSlot c xs 
  buscarSlot _ [] = Nothing

  actualizarLista :: [a] -> Int -> a -> [a]
  actualizarLista lista indice objeto = take indice lista ++ [objeto] ++ drop (indice+1) lista
