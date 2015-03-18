#!/usr/bin/python
# -*- coding: iso-8859-1 -*-

# run this as > $ python lex.py input.rs

import sys
from collections import OrderedDict

inFile = sys.argv[1]

reservadas = ["module","box","input","output","t_signal","p_signal","var","initially","up","activate","on_exception","emit"]
operadores = ["===>","--->","=",">","<",">=","<=","+","-","&"]
separadores = ["[","]",",",":","#","(",")","."]

arq = open("output.txt","wb")

with open(inFile,'r') as f:
	palavra = []
	reservada = []
	separador = []
	operador = []
	indicador = []
	digito = []
	lista = []
	tokens = []
	row = 1
	row_l = []
	col = 0
	last_col = 0
	while True:
		caracter = f.read(1)
		col += 1
		if caracter == "\n":
			row += 1
			last_col = col
			col = 0
		if caracter != " " and caracter != "\n" and caracter != "\r":
			palavra.append(caracter)
		else:
			lista.append(palavra)
			palavra = []
		if not caracter: 
			col = last_col-1
			row -= 1
			break


	for word in lista:
		#print word
		for d in separadores:
			if d in word:
				tokens.append(d)
				separador.append(d)
				word = [w.replace(d, "|") for w in word]
		for d in operadores:
			if d in word:
				tokens.append(d)
				operador.append(d)
				word = [w.replace(d, "|") for w in word]
		word = "".join(word)
		words = word.split("|")
		for w in words:
			if w in reservadas:
				tokens.append(w)
				reservada.append(w)
				words.remove(w)
			elif w in operadores:
				tokens.append(w)
				operador.append(w)
				words.remove(w)
			else:
				tokens.append(w)
				indicador.append(w)
				words.remove(w)
		for o in indicador:
			if o == "":
				indicador.remove(o)
			if o.isdigit():
				digito.append(o)
				indicador.remove(o)
		for t in tokens:
			if t == "":
				tokens.remove(t)
	
	tokens = list(OrderedDict.fromkeys(tokens))
	for t in tokens: 
		if t in reservada:
			print t + " - palavra reservada"
			arq.write(t + " - palavra reservada\n")
		elif t in indicador:
			print t + " - indicador"
			arq.write(t + " - indicador\n")
		elif t in separador:
			print t + " - separador"
			arq.write(t + " - separador\n")
		elif t in digito:
			print t + " - constante numérica"
			arq.write(t + " - constante numérica\n")
		else:
			print t + " - operador"
			arq.write(t + " - operador\n")

arq.close()