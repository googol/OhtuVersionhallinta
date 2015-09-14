# OhtuVersionhallinta
This program is an exercise for the 2015 course Ohjelmistotuotanto (Software engineering) at the department of computer science in the University of Helsinki.

It is meant to be a proof-of-concept version control system, with extremely simplified operations. It is written in Rust, and acts also as a Rust exercise for me (it's my first Rust program).

# Dependencies
Building the project requires rustc and cargo. Stable rust 1.2 was used for development. You can get Rust from: https://www.rust-lang.org/

# Building
Enter the project directory and build the program using the command "cargo build --relese".

The built program will be at "target\\release\\ohtuv"

# Usage
ohtuv \<command>, where \<command> is one of:
* init : initializes the repository in the current directory. Creates a directory ".ohtuv" to hold the repository.
* find : finds the current repository path by walking up the directory tree.
* save \<file> : Saves the given file to the repository. \<file> must point to an existing file, not a directory. An initialized repository needs to exist in the current directory or any of its ancestors.

# Original assignment (in Finnish)
Ohjelmistokehittäjän yleissivistykseen kuuluu myös tietää, miten git toimii sisäiseltä rakenteeltaan. Lue https://git-scm.com/book/en/v2/Git-Internals-Git-Objects. 

Git on melko monimutkainen joten emme ota suoraan mallia siitä, vaan koodaamme oman yksinkertaisemman  OhtuVersionhallinta -ohjelman. Ohjelmointikieli on vapaa ja speksit ovat: 

* <code>ohtuv init</code> luo .ohtuv -kansion nykyiseen kansioon jos sitä ei ole 
* <code>ohtuv save tiedostonnimi</code> tallentaa tiedostosta kopion .ohtuv -kansioon. Tiedostolle annetaan nimeksi timestamp ja tiedostonnimi yhdistettynä tyyliin "6.9.2015.15:34.ekatiedosto". Jos nykyisessä hakemistossa ei ole .ohtuv -kansiota, ohjelma lähtee hakemaan ylähakemistoista kansiota ja tallentaa kopion ensimmäisenä löytyvään. Jos .ohtuv -kansiota ei löydy edes juuresta, ohjelma ilmoittaa ettei löytänyt ohtuv -repositoriota. 
* esimerkiksi <code>ohtuv restore 6.9.2015 ekatiedosto</code> palauttaa 6.9.2015.15:34.ekatiedosto -tiedoston nykyiseksi ekatiedostoksi. Jos päivältä 6.9.2015 on useita eri kopioita, ohjelma listaa jokaisen niistä nimen ja pyytää käyttäjää spesifioimaan tarkemmin. 
