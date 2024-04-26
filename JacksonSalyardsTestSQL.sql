
CREATE TABLE Movie (
  movieId INT PRIMARY KEY,
  title VARCHAR(500),
  releaseDate INT,
  format VARCHAR(10),
  description VARCHAR(1000)
);

CREATE TABLE CastMembers (
  castID INT PRIMARY KEY,
  age INT,
  name VARCHAR(500),
  role VARCHAR(500),
  mis VARCHAR(1000),
  movieId INT,
  FOREIGN KEY (movieId) REFERENCES Movie(movieId)
);

CREATE TABLE Review (
  reviewID INT PRIMARY KEY,
  aggregate INT,
  sub_review_num INT,
  movieId INT,
  FOREIGN KEY (movieId) REFERENCES Movie(movieId)
);

CREATE TABLE Sub_Review (
  reviewID INT,
  subreviewID INT PRIMARY KEY,
  sub_review_title VARCHAR(50),
  sub_review_score INT,
  sub_review_desc VARCHAR(1000),
  FOREIGN KEY (reviewID) REFERENCES Review(reviewID)
);

INSERT INTO Movie (movieId, title, releaseDate, format, description)
VALUES
  (1, 'Inception', 2010, 'Blu-ray', 'A mind-bending heist thriller directed by Christopher Nolan'),
  (2, 'The Shawshank Redemption', 1994, 'DVD', 'A powerful drama based on Stephen King\'s novella'),
  (3, 'Inglourious Basterds', 2009, 'Blu-ray', 'A Quentin Tarantino film set during World War II'),
  (5, 'Bullet Train', 2022, 'Digital', 'An action thriller film directed by David Leitch');

-- Insert cast members
INSERT INTO CastMembers (castID, age, name, role, mis, movieId)
VALUES
  (1, 45, 'Leonardo DiCaprio', 'Cobb', 'An experienced thief skilled in entering the subconscious', 1),
  (2, 52, 'Joseph Gordon-Levitt', 'Arthur', 'A skilled point man and close friend of Cobb', 1),
  (3, 60, 'Morgan Freeman', 'Ellis Boyd "Red" Redding', 'A longtime inmate at Shawshank State Penitentiary', 2),
  (4, 34, 'Tim Robbins', 'Andy Dufresne', 'A banker wrongly convicted of murder', 2),
  (5, 43, 'Brad Pitt', 'Lt. Aldo Raine', 'Leader of the Basterds, a group of Jewish-American soldiers', 3),
  (6, 36, 'Diane Kruger', 'Bridget von Hammersmark', 'A German film actress and undercover agent', 3),
  (7, 55, 'Christoph Waltz', 'Col. Hans Landa', 'A cunning and ruthless SS officer known as "The Jew Hunter"', 3),
  (8, 58, 'Brad Pitt', 'Ladybug', 'A skilled assassin with a mysterious past', 5),
  (9, 42, 'Zazie Beetz', 'Lemon', 'A deadly and unpredictable assassin', 5),
  (10, 36, 'Michael Shannon', 'Tangerine', 'A seasoned hitman with a personal agenda', 5);

-- Insert reviews
INSERT INTO Review (reviewID, aggregate, sub_review_num, movieId)
VALUES
  (1, 4.5, 2, 1),
  (2, 4.8, 1, 2),
  (3, 4.2, 3, 3),
  (5, 4.6, 2, 5);

-- Insert sub-reviews
INSERT INTO Sub_Review (reviewID, subreviewID, sub_review_title, sub_review_score, sub_review_desc)
VALUES
 (1, 1, 'Great Plot', 4, 'The storyline is intricate and keeps you on the edge of your seat'),
  (1, 2, 'Amazing Cinematography', 5, 'The visuals are stunning, and the special effects are top-notch'),
  (2, 3, 'Emotional Journey', 4.8, 'The emotional depth of the characters is unparalleled'),
  (3, 4, 'Outstanding Performances', 4.5, 'The cast, especially Christoph Waltz, delivered exceptional performances'),
  (3, 5, 'Unique Storytelling', 4, 'Tarantino\'s signature nonlinear storytelling adds a unique twist to the war genre'),
  (3, 6, 'Intense Action Sequences', 4.2, 'The action scenes are gripping and well-executed'),
  (5, 7, 'High-Octane Action', 4.8, 'The film delivers intense and well-choreographed action sequences'),
  (5, 8, 'Star-Studded Cast', 4.4, 'Brad Pitt and the ensemble cast contribute to the movie\'s appeal');