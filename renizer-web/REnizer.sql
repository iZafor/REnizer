-- Enable Warnings
\W;

-- Drop tables if exists
DROP TABLE IF EXISTS Collaboration_Task_T;
DROP TABLE IF EXISTS Collaboration_T;
DROP TABLE IF EXISTS Investor_Invest_Project_T;
DROP TABLE IF EXISTS Investment_Proposal_T;
DROP TABLE IF EXISTS Project_T; 
DROP TABLE IF EXISTS Project_Contributor_Skill_T;
DROP TABLE IF EXISTS Project_Contributor_T;
DROP TABLE IF EXISTS Project_Manager_T; 
DROP TABLE IF EXISTS Project_Associate_T;
DROP TABLE IF EXISTS Investor_T; 
DROP TABLE IF EXISTS User_T; 
DROP TABLE IF EXISTS Organization_Contact_T; 
DROP TABLE IF EXISTS Organization_T; 

-- Create tables
CREATE TABLE Organization_T(
	org_id CHAR(8) NOT NULL,
	name VARCHAR(50) NOT NULL,
	email VARCHAR(30),
	location VARCHAR(50),
	CONSTRAINT Organization_PK PRIMARY KEY (org_id)
);

CREATE TABLE Organization_Contact_T(
	org_id CHAR(8) NOT NULL,
	contact VARCHAR(15) NOT NULL,
	CONSTRAINT Organization_Contact_PK PRIMARY KEY (org_id, contact),
	CONSTRAINT Organization_Contact_FK FOREIGN KEY (org_id) REFERENCES Organization_T(org_id)
);

CREATE TABLE User_T(
	user_id CHAR(8) NOT NULL,
	first_name VARCHAR(20) NOT NULL,
	last_name VARCHAR(20),
	password VARCHAR(255),
	email VARCHAR(30) NOT NULL,
	contact_number VARCHAR(15),
	user_type CHAR(1) NOT NULL,
	org_id CHAR(8),
	CONSTRAINT User_PK PRIMARY KEY (user_id),
	CONSTRAINT User_FK FOREIGN KEY (org_id) REFERENCES Organization_T(org_id)
);

CREATE TABLE Investor_T(
	i_user_id CHAR(8) NOT NULL,
	investor_type VARCHAR(20) NOT NULL,
	CONSTRAINT Investor_PK PRIMARY KEY (i_user_id),
	CONSTRAINT Investor_FK FOREIGN KEY (i_user_id) REFERENCES User_T(user_id)
);

CREATE TABLE Project_Associate_T(
	p_user_id CHAR(8) NOT NULL,
	hourly_rate NUMERIC(6, 2),
	working_experience NUMERIC(2, 0),
	associate_type CHAR(1) NOT NULL,
	CONSTRAINT Project_Associate_PK PRIMARY KEY (p_user_id),
	CONSTRAINT Project_Associate_FK FOREIGN KEY (p_user_id) REFERENCES User_T(user_id)	
);

CREATE TABLE Project_Manager_T(
	m_p_user_id CHAR(8) NOT NULL,
	CONSTRAINT Project_Manager_PK PRIMARY KEY (m_p_user_id),
	CONSTRAINT Project_Manager_FK FOREIGN KEY (m_p_user_id) REFERENCES Project_Associate_T(p_user_id)	
);

CREATE TABLE Project_Contributor_T(
	c_p_user_id CHAR(8) NOT NULL,
	working_department VARCHAR(50),
	CONSTRAINT Project_Contributor_PK PRIMARY KEY (c_p_user_id),
	CONSTRAINT Project_Contributor_FK FOREIGN KEY (c_p_user_id) REFERENCES Project_Associate_T(p_user_id)	
);

CREATE TABLE Project_Contributor_Skill_T(
	c_p_user_id CHAR(8) NOT NULL,
	skill VARCHAR(50) NOT NULL,
	CONSTRAINT Project_Contributor_Skill_PK PRIMARY KEY (c_p_user_id, skill),
	CONSTRAINT Project_Contributor_Skill_FK FOREIGN KEY (c_p_user_id) REFERENCES Project_Contributor_T(c_p_user_id)	
);

CREATE TABLE Project_T(
	project_id CHAR(8) NOT NULL,
	name VARCHAR(80) NOT NULL,
	description VARCHAR(200),
	location VARCHAR(50),
	start_date DATETIME,
	end_date DATETIME,
	status VARCHAR(20),
	energy_rate_kwh NUMERIC(5, 2),
	produced_energy_kwh NUMERIC(8, 2),
	energy_sold_kwh NUMERIC(8, 2),
	total_cost NUMERIC(15, 2),
	org_restricted BOOLEAN NOT NULL,
	m_p_user_id CHAR(8),
	creation_date DATETIME NOT NULL,
	CONSTRAINT Project_PK PRIMARY KEY (project_id),
	CONSTRAINT Project_FK FOREIGN KEY (m_p_user_id) REFERENCES Project_Manager_T(m_p_user_id)
);

CREATE TABLE Investment_Proposal_T(
	i_user_id CHAR(8) NOT NULL,	
	project_id CHAR(8) NOT NULL,
	investment_amount NUMERIC(15, 2) NOT NULL,
	proposal_date DATETIME NOT NULL,
	proposal_status VARCHAR(20) NOT NULL,
	CONSTRAINT Investment_Proposal_PK PRIMARY KEY (i_user_id, project_id, proposal_date),
	CONSTRAINT Investment_Proposal_FK1 FOREIGN KEY (i_user_id) REFERENCES Investor_T(i_user_id),
	CONSTRAINT Investment_Proposal_FK2 FOREIGN KEY (project_id) REFERENCES Project_T(project_id)	
);

CREATE TABLE Investor_Invest_Project_T(
	i_user_id CHAR(8) NOT NULL,	
	project_id CHAR(8) NOT NULL,
	investment_amount NUMERIC(15, 2) NOT NULL,
	investment_date DATETIME NOT NULL,
	CONSTRAINT Investor_Invest_Project_PK PRIMARY KEY (i_user_id, project_id, investment_date),
	CONSTRAINT Investor_Invest_Project_FK1 FOREIGN KEY (i_user_id) REFERENCES Investor_T(i_user_id),
	CONSTRAINT Investor_Invest_Project_FK2 FOREIGN KEY (project_id) REFERENCES Project_T(project_id)	
);

CREATE TABLE Collaboration_T(
	p_user_id CHAR(8) NOT NULL,
	project_id CHAR(8) NOT NULL,
	start_date DATETIME NOT NULL,
	end_date DATETIME,
	role VARCHAR(50) NOT NULL,
	CONSTRAINT Collaboration_PK PRIMARY KEY (p_user_id, project_id, start_date, role),
	CONSTRAINT Collaboration_FK1 FOREIGN KEY (p_user_id) REFERENCES Project_Associate_T(p_user_id),
	CONSTRAINT Collaboration_FK2 FOREIGN KEY (project_id) REFERENCES Project_T(project_id)	
);

CREATE TABLE Collaboration_Task_T(
	p_user_id CHAR(8) NOT NULL,
	project_id CHAR(8) NOT NULL,
	task_name VARCHAR(50) NOT NULL,
	status VARCHAR(20),
	assigned_date DATETIME NOT NULL,
	start_date DATETIME NOT NULL,
	delivery_date DATETIME,
	expected_hour NUMERIC(4, 1) NOT NULL,
	hour_taken NUMERIC(4, 1),
	expected_day NUMERIC(3) NOT NULL,
	CONSTRAINT Collaboration_Task_PK PRIMARY KEY (p_user_id, project_id, task_name, assigned_date),
	CONSTRAINT Collaboration_Task_FK1 FOREIGN KEY (p_user_id) REFERENCES Collaboration_T(p_user_id),
	CONSTRAINT Collaboration_Task_FK2 FOREIGN KEY (project_id) REFERENCES Collaboration_T(project_id)
);

-- Delete Entries
DELETE FROM Collaboration_Task_T;
DELETE FROM Collaboration_T;
DELETE FROM Investor_Invest_Project_T;
DELETE FROM Investment_Proposal_T;
DELETE FROM Project_T;
DELETE FROM Project_Contributor_Skill_T;
DELETE FROM Project_Contributor_T;
DELETE FROM Project_Manager_T;
DELETE FROM Project_Associate_T;
DELETE FROM Investor_T;
DELETE FROM User_T;
DELETE FROM Organization_Contact_T;
DELETE FROM Organization_T;

-- Populate Tables

-- Organization_T (10 Entries)
INSERT INTO Organization_T (org_id, name, email, location) 
VALUES 
('b3e49ca6', 'Tech Solutions Inc.', 'info@techsolutions.com', 'San Francisco, CA'),
('b3e4a86f', 'Global Logistics Group', 'contact@globallogistics.com', 'New York, NY'),
('b3e4aae9', 'Healthcare Innovations Ltd.', 'info@healthinnovate.com', 'London, UK'),
('b3e4ad35', 'Green Energy Corporation', 'info@greenenergycorp.com', 'Berlin, Germany'),
('b3e4b115', 'Creative Media Studios', 'contact@creativemedia.com', 'Los Angeles, CA'),
('b3e4b42c', 'Financial Solutions Ltd.', 'info@financesolutions.com', 'Singapore'),
('b3e4b729', 'Manufacturing Innovations Inc.', 'info@manufactureinnovate.com', 'Tokyo, Japan'),
('b3e4b995', 'Education Empowerment Foundation', 'info@eduempower.org', 'New Delhi, India'),
('b3e4bbdf', 'Consulting Experts Group', 'contact@consultexperts.com', 'Sydney, Australia'),
('b3e4be72', 'Food For All NGO', 'info@foodforallngo.org', 'Nairobi, Kenya');

-- Organization_Contact_T (10 Entries)
INSERT INTO Organization_Contact_T (org_id, contact) 
VALUES 
('b3e49ca6', '123-456-7890'),
('b3e4a86f', '234-567-8901'),
('b3e4aae9', '345-678-9012'),
('b3e4ad35', '456-789-0123'),
('b3e4b115', '567-890-1234'),
('b3e4b42c', '678-901-2345'),
('b3e4b729', '789-012-3456'),
('b3e4b995', '890-123-4567'),
('b3e4bbdf', '901-234-5678'),
('b3e4be72', '012-345-6789');

-- User_T (50 Entries - 10 Investors, 40 Associates)
INSERT INTO User_T (user_id, password, first_name, last_name, email, contact_number, user_type, org_id)
VALUES 
('4e6ae6d3', '$2b$12$N.RbIrRGzbGVnjzwWgyQTeW/l.RUa9eVQjAx3sEpZKO9NJV0bVDE6', 'John', 'Doe', 'john@example.com', '123-456-7890', 'I', NULL),
('4e6aece9', '$2b$12$N.RbIrRGzbGVnjzwWgyQTeW/l.RUa9eVQjAx3sEpZKO9NJV0bVDE6', 'Jane', 'Smith', 'jane@example.com', '234-567-8901', 'I', NULL),
('4e6af275', '$2b$12$N.RbIrRGzbGVnjzwWgyQTeW/l.RUa9eVQjAx3sEpZKO9NJV0bVDE6', 'Michael', 'Johnson', 'michael@example.com', '345-678-9012', 'I', NULL),
('4e6af650', '$2b$12$N.RbIrRGzbGVnjzwWgyQTeW/l.RUa9eVQjAx3sEpZKO9NJV0bVDE6', 'Emily', 'Brown', 'emily@example.com', '456-789-0123', 'I', NULL),
('4e6afa6a', '$2b$12$N.RbIrRGzbGVnjzwWgyQTeW/l.RUa9eVQjAx3sEpZKO9NJV0bVDE6', 'David', 'Lee', 'david@example.com', '567-890-1234', 'I', NULL),
('4e6afef6', '$2b$12$N.RbIrRGzbGVnjzwWgyQTeW/l.RUa9eVQjAx3sEpZKO9NJV0bVDE6', 'Sarah', 'Wilson', 'sarah@example.com', '678-901-2345', 'I', NULL),
('4e6b02b3', '$2b$12$N.RbIrRGzbGVnjzwWgyQTeW/l.RUa9eVQjAx3sEpZKO9NJV0bVDE6', 'Alex', 'Kim', 'alex@example.com', '789-012-3456', 'I', NULL),
('4e6b075f', '$2b$12$N.RbIrRGzbGVnjzwWgyQTeW/l.RUa9eVQjAx3sEpZKO9NJV0bVDE6', 'Jessica', 'Garcia', 'jessica@example.com', '890-123-4567', 'I', NULL),
('4e6b09b3', '$2b$12$N.RbIrRGzbGVnjzwWgyQTeW/l.RUa9eVQjAx3sEpZKO9NJV0bVDE6', 'Daniel', 'Martinez', 'daniel@example.com', '901-234-5678', 'I', NULL),
('4e6b0bf3', '$2b$12$N.RbIrRGzbGVnjzwWgyQTeW/l.RUa9eVQjAx3sEpZKO9NJV0bVDE6', 'Emma', 'Taylor', 'emma@example.com', '012-345-6789', 'I', NULL),
('4e6b0f17', '$2b$12$N.RbIrRGzbGVnjzwWgyQTeW/l.RUa9eVQjAx3sEpZKO9NJV0bVDE6', 'William', 'Johnson', 'william@example.com', '111-222-3333', 'P', 'b3e49ca6'),
('4e6b1152', '$2b$12$N.RbIrRGzbGVnjzwWgyQTeW/l.RUa9eVQjAx3sEpZKO9NJV0bVDE6', 'Olivia', 'Brown', 'olivia@example.com', '222-333-4444', 'P', 'b3e49ca6'),
('4e6b138f', '$2b$12$N.RbIrRGzbGVnjzwWgyQTeW/l.RUa9eVQjAx3sEpZKO9NJV0bVDE6', 'James', 'Wilson', 'james@example.com', '333-444-5555', 'P', 'b3e49ca6'),
('4e6b15c6', '$2b$12$N.RbIrRGzbGVnjzwWgyQTeW/l.RUa9eVQjAx3sEpZKO9NJV0bVDE6', 'Sophia', 'Anderson', 'sophia@example.com', '444-555-6666', 'P', 'b3e49ca6'),
('4e6b18e4', '$2b$12$N.RbIrRGzbGVnjzwWgyQTeW/l.RUa9eVQjAx3sEpZKO9NJV0bVDE6', 'Benjamin', 'Lee', 'benjamin@example.com', '555-666-7777', 'P', 'b3e49ca6'),
('4e6b1b2e', '$2b$12$N.RbIrRGzbGVnjzwWgyQTeW/l.RUa9eVQjAx3sEpZKO9NJV0bVDE6', 'Ava', 'Martinez', 'ava@example.com', '666-777-8888', 'P', 'b3e49ca6'),
('4e6b1d60', '$2b$12$N.RbIrRGzbGVnjzwWgyQTeW/l.RUa9eVQjAx3sEpZKO9NJV0bVDE6', 'Logan', 'Garcia', 'logan@example.com', '777-888-9999', 'P', 'b3e49ca6'),
('4e6b2035', '$2b$12$N.RbIrRGzbGVnjzwWgyQTeW/l.RUa9eVQjAx3sEpZKO9NJV0bVDE6', 'Mia', 'Lopez', 'mia@example.com', '888-999-0000', 'P', 'b3e49ca6'),
('4e6b2397', '$2b$12$N.RbIrRGzbGVnjzwWgyQTeW/l.RUa9eVQjAx3sEpZKO9NJV0bVDE6', 'Elijah', 'Hernandez', 'elijah@example.com', '999-000-1111', 'P', 'b3e49ca6'),
('4e6b26a9', '$2b$12$N.RbIrRGzbGVnjzwWgyQTeW/l.RUa9eVQjAx3sEpZKO9NJV0bVDE6', 'Charlotte', 'Young', 'charlotte@example.com', '000-111-2222', 'P', 'b3e49ca6'),
('4e6b28ee', '$2b$12$N.RbIrRGzbGVnjzwWgyQTeW/l.RUa9eVQjAx3sEpZKO9NJV0bVDE6', 'Amelia', 'Wright', 'amelia@example.com', '111-222-3333', 'P', 'b3e4a86f'),
('4e6b2b4b', '$2b$12$N.RbIrRGzbGVnjzwWgyQTeW/l.RUa9eVQjAx3sEpZKO9NJV0bVDE6', 'William', 'Martin', 'william.m@example.com', '222-333-4444', 'P', 'b3e4a86f'),
('4e6b2d84', '$2b$12$N.RbIrRGzbGVnjzwWgyQTeW/l.RUa9eVQjAx3sEpZKO9NJV0bVDE6', 'James', 'Allen', 'james.a@example.com', '333-444-5555', 'P', 'b3e4a86f'),
('4e6b2fbb', '$2b$12$N.RbIrRGzbGVnjzwWgyQTeW/l.RUa9eVQjAx3sEpZKO9NJV0bVDE6', 'Sophia', 'White', 'sophia.w@example.com', '444-555-6666', 'P', 'b3e4a86f'),
('4e6b3260', '$2b$12$N.RbIrRGzbGVnjzwWgyQTeW/l.RUa9eVQjAx3sEpZKO9NJV0bVDE6', 'Benjamin', 'Lewis', 'benjamin.l@example.com', '555-666-7777', 'P', 'b3e4a86f'),
('4e6b349d', '$2b$12$N.RbIrRGzbGVnjzwWgyQTeW/l.RUa9eVQjAx3sEpZKO9NJV0bVDE6', 'Ethan', 'Thompson', 'ethan.t@example.com', '666-777-8888', 'P', 'b3e4a86f'),
('4e6b372f', '$2b$12$N.RbIrRGzbGVnjzwWgyQTeW/l.RUa9eVQjAx3sEpZKO9NJV0bVDE6', 'Mia', 'Adams', 'mia.a@example.com', '777-888-9999', 'P', 'b3e4a86f'),
('4e6b396b', '$2b$12$N.RbIrRGzbGVnjzwWgyQTeW/l.RUa9eVQjAx3sEpZKO9NJV0bVDE6', 'Olivia', 'Clark', 'olivia.c@example.com', '888-999-0000', 'P', 'b3e4a86f'),
('4e6b3be6', '$2b$12$N.RbIrRGzbGVnjzwWgyQTeW/l.RUa9eVQjAx3sEpZKO9NJV0bVDE6', 'Elijah', 'Allen', 'elijah.a@example.com', '999-000-1111', 'P', 'b3e4a86f'),
('4e6b3f60', '$2b$12$N.RbIrRGzbGVnjzwWgyQTeW/l.RUa9eVQjAx3sEpZKO9NJV0bVDE6', 'Charlotte', 'Hernandez', 'charlotte.h@example.com', '000-111-2222', 'P', 'b3e4a86f'),
('4e6b41a8', '$2b$12$N.RbIrRGzbGVnjzwWgyQTeW/l.RUa9eVQjAx3sEpZKO9NJV0bVDE6', 'William', 'Wright', 'william.w@example.com', '111-222-3333', 'P', 'b3e4aae9'),
('4e6b43db', '$2b$12$N.RbIrRGzbGVnjzwWgyQTeW/l.RUa9eVQjAx3sEpZKO9NJV0bVDE6', 'James', 'Taylor', 'james.t@example.com', '222-333-4444', 'P', 'b3e4aae9'),
('4e6b46aa', '$2b$12$N.RbIrRGzbGVnjzwWgyQTeW/l.RUa9eVQjAx3sEpZKO9NJV0bVDE6', 'Sophia', 'Lopez', 'sophia.l@example.com', '333-444-5555', 'P', 'b3e4aae9'),
('4e6b4998', '$2b$12$N.RbIrRGzbGVnjzwWgyQTeW/l.RUa9eVQjAx3sEpZKO9NJV0bVDE6', 'Benjamin', 'Young', 'benjamin.y@example.com', '444-555-6666', 'P', 'b3e4aae9'),
('4e6b4c84', '$2b$12$N.RbIrRGzbGVnjzwWgyQTeW/l.RUa9eVQjAx3sEpZKO9NJV0bVDE6', 'Ethan', 'Martin', 'ethan.m@example.com', '555-666-7777', 'P', 'b3e4aae9'),
('4e6b4f08', '$2b$12$N.RbIrRGzbGVnjzwWgyQTeW/l.RUa9eVQjAx3sEpZKO9NJV0bVDE6', 'Mia', 'Allen', 'mia.al@example.com', '666-777-8888', 'P', 'b3e4aae9'),
('4e6b5530', '$2b$12$N.RbIrRGzbGVnjzwWgyQTeW/l.RUa9eVQjAx3sEpZKO9NJV0bVDE6', 'Olivia', 'White', 'olivia.w@example.com', '777-888-9999', 'P', 'b3e4aae9'),
('4e6b57e6', '$2b$12$N.RbIrRGzbGVnjzwWgyQTeW/l.RUa9eVQjAx3sEpZKO9NJV0bVDE6', 'Elijah', 'Lewis', 'elijah.l@example.com', '888-999-0000', 'P', 'b3e4aae9'),
('4e6b5a14', '$2b$12$N.RbIrRGzbGVnjzwWgyQTeW/l.RUa9eVQjAx3sEpZKO9NJV0bVDE6', 'Charlotte', 'Thompson', 'charlotte.t@example.com', '999-000-1111', 'P', 'b3e4aae9'),
('4e6b5ce4', '$2b$12$N.RbIrRGzbGVnjzwWgyQTeW/l.RUa9eVQjAx3sEpZKO9NJV0bVDE6', 'William', 'Adams', 'william.a@example.com', '000-111-2222', 'P', 'b3e4aae9'),
('4e6b5fbf', '$2b$12$N.RbIrRGzbGVnjzwWgyQTeW/l.RUa9eVQjAx3sEpZKO9NJV0bVDE6', 'James', 'Clark', 'james.c@example.com', '111-222-3333', 'P', 'b3e4aae9'),
('4e6b6242', '$2b$12$N.RbIrRGzbGVnjzwWgyQTeW/l.RUa9eVQjAx3sEpZKO9NJV0bVDE6', 'Sophia', 'Allen', 'sophia.al@example.com', '222-333-4444', 'P', 'b3e4aae9'),
('4e6b6491', '$2b$12$N.RbIrRGzbGVnjzwWgyQTeW/l.RUa9eVQjAx3sEpZKO9NJV0bVDE6', 'Benjamin', 'Hernandez', 'benjamin.h@example.com', '333-444-5555', 'P', 'b3e4aae9'),
('4e6b673b', '$2b$12$N.RbIrRGzbGVnjzwWgyQTeW/l.RUa9eVQjAx3sEpZKO9NJV0bVDE6', 'Ethan', 'Wright', 'ethan.w@example.com', '444-555-6666', 'P', 'b3e4aae9'),
('4e6b696c', '$2b$12$N.RbIrRGzbGVnjzwWgyQTeW/l.RUa9eVQjAx3sEpZKO9NJV0bVDE6', 'Mia', 'Taylor', 'mia.t@example.com', '555-666-7777', 'P', 'b3e4aae9'),
('4e6b6bb6', '$2b$12$N.RbIrRGzbGVnjzwWgyQTeW/l.RUa9eVQjAx3sEpZKO9NJV0bVDE6', 'Olivia', 'Lopez', 'olivia.l@example.com', '666-777-8888', 'P', 'b3e4aae9'),
('4e6b6de7', '$2b$12$N.RbIrRGzbGVnjzwWgyQTeW/l.RUa9eVQjAx3sEpZKO9NJV0bVDE6', 'Elijah', 'Young', 'elijah.y@example.com', '777-888-9999', 'P', 'b3e4aae9'),
('4e6b7011', '$2b$12$N.RbIrRGzbGVnjzwWgyQTeW/l.RUa9eVQjAx3sEpZKO9NJV0bVDE6', 'Charlotte', 'Martin', 'charlotte.m@example.com', '888-999-0000', 'P', 'b3e4aae9'),
('4e6b7243', '$2b$12$N.RbIrRGzbGVnjzwWgyQTeW/l.RUa9eVQjAx3sEpZKO9NJV0bVDE6', 'William', 'Allen', 'william.al@example.com', '999-000-1111', 'P', 'b3e4aae9'),
('4e6b74f1', '$2b$12$N.RbIrRGzbGVnjzwWgyQTeW/l.RUa9eVQjAx3sEpZKO9NJV0bVDE6', 'James', 'Thompson', 'james.th@example.com', '000-111-2222', 'P', 'b3e4aae9');


-- Investor_T (10 Entries)
INSERT INTO Investor_T (i_user_id, investor_type) 
VALUES 
('4e6ae6d3', 'Individual'),
('4e6aece9', 'Individual'),
('4e6af275', 'Individual'),
('4e6af650', 'Individual'),
('4e6afa6a', 'Individual'),
('4e6afef6', 'Individual'),
('4e6b02b3', 'Individual'),
('4e6b075f', 'Individual'),
('4e6b09b3', 'Individual'),
('4e6b0bf3', 'Individual');

-- Project_Associate_T (40 Entries - 10 Managers, 30 Contributors)
-- Managers
INSERT INTO Project_Associate_T (p_user_id, hourly_rate, working_experience, associate_type)
VALUES 
('4e6b0f17', 55.00, 7, 'M'),
('4e6b1152', 60.00, 6, 'M'),
('4e6b138f', 65.00, 5, 'M'),
('4e6b15c6', 70.00, 4, 'M'),
('4e6b18e4', 75.00, 3, 'M'),
('4e6b1b2e', 80.00, 2, 'M'),
('4e6b1d60', 85.00, 1, 'M'),
('4e6b2035', 90.00, 8, 'M'),
('4e6b2397', 95.00, 7, 'M'),
('4e6b26a9', 59.00, 5, 'M');

-- Contributors
INSERT INTO Project_Associate_T (p_user_id, hourly_rate, working_experience, associate_type)
VALUES 
('4e6b28ee', 32.00, 4, 'C'),
('4e6b2b4b', 35.00, 3, 'C'),
('4e6b2d84', 40.00, 2, 'C'),
('4e6b2fbb', 42.00, 1, 'C'),
('4e6b3260', 45.00, 8, 'C'),
('4e6b349d', 50.00, 7, 'C'),
('4e6b372f', 55.00, 6, 'C'),
('4e6b396b', 60.00, 5, 'C'),
('4e6b3be6', 65.00, 4, 'C'),
('4e6b3f60', 70.00, 3, 'C'),
('4e6b41a8', 75.00, 2, 'C'),
('4e6b43db', 80.00, 1, 'C'),
('4e6b46aa', 85.00, 8, 'C'),
('4e6b4998', 90.00, 7, 'C'),
('4e6b4c84', 95.00, 6, 'C'),
('4e6b4f08', 100.00, 5, 'C'),
('4e6b5530', 32.00, 4, 'C'),
('4e6b57e6', 35.00, 3, 'C'),
('4e6b5a14', 40.00, 2, 'C'),
('4e6b5ce4', 42.00, 1, 'C'),
('4e6b5fbf', 45.00, 8, 'C'),
('4e6b6242', 50.00, 7, 'C'),
('4e6b6491', 55.00, 6, 'C'),
('4e6b673b', 60.00, 5, 'C'),
('4e6b696c', 65.00, 4, 'C'),
('4e6b6bb6', 70.00, 3, 'C'),
('4e6b6de7', 75.00, 2, 'C'),
('4e6b7011', 80.00, 1, 'C'),
('4e6b7243', 85.00, 8, 'C'),
('4e6b74f1', 90.00, 7, 'C');

-- Project_Manager_T (10 Entries)
INSERT INTO Project_Manager_T (m_p_user_id) 
VALUES 
('4e6b0f17'),
('4e6b1152'),
('4e6b138f'),
('4e6b15c6'),
('4e6b18e4'),
('4e6b1b2e'),
('4e6b1d60'),
('4e6b2035'),
('4e6b2397'),
('4e6b26a9');

-- Project_Contributor_T (30 Entries)
INSERT INTO Project_Contributor_T (c_p_user_id, working_department)
VALUES 
('4e6b28ee', 'Development'),
('4e6b2b4b', 'Engineering'),
('4e6b2d84', 'Procurement'),
('4e6b2fbb', 'Construction'),
('4e6b3260', 'Operations'),
('4e6b349d', 'Finance'),
('4e6b372f', 'Regulatory Affairs'),
('4e6b396b', 'Legal'),
('4e6b3be6', 'Public Affairs'),
('4e6b3f60', 'Land Management'),
('4e6b41a8', 'Environmental'),
('4e6b43db', 'Sustainability'),
('4e6b46aa', 'Human Resources'),
('4e6b4998', 'Information Technology (IT)'),
('4e6b4c84', 'Security'),
('4e6b4f08', 'Health and Safety'),
('4e6b5530', 'Quality Assurance/Quality Control (QA/QC)'),
('4e6b57e6', 'Research and Development (R&D)'),
('4e6b5a14', 'Marketing and Sales'),
('4e6b5ce4', 'Customer Service'),
('4e6b5fbf', 'Community Engagement'),
('4e6b6242', 'Development'),
('4e6b6491', 'Engineering'),
('4e6b673b', 'Procurement'),
('4e6b696c', 'Construction'),
('4e6b6bb6', 'Operations'),
('4e6b6de7', 'Finance'),
('4e6b7011', 'Regulatory Affairs'),
('4e6b7243', 'Legal'),
('4e6b74f1', 'Public Affairs');

-- Project_Contributor_Skill_T (91 Entries)
INSERT INTO Project_Contributor_Skill_T (c_p_user_id, skill)
VALUES 
-- User 4e6b28ee
('4e6b28ee', 'Project Management'),
('4e6b28ee', 'Team Leadership'),
('4e6b28ee', 'Communication'),
('4e6b28ee', 'Problem Solving'),
-- User 4e6b2b4b
('4e6b2b4b', 'Civil Engineering'),
('4e6b2b4b', 'Structural Analysis'),
('4e6b2b4b', 'AutoCAD'),
-- User 4e6b2d84
('4e6b2d84', 'Supply Chain Management'),
('4e6b2d84', 'Vendor Negotiation'),
('4e6b2d84', 'Inventory Management'),
-- User 4e6b2fbb
('4e6b2fbb', 'Construction Management'),
('4e6b2fbb', 'Safety Compliance'),
('4e6b2fbb', 'Project Scheduling'),
-- User 4e6b3260
('4e6b3260', 'Operations Management'),
('4e6b3260', 'Logistics'),
('4e6b3260', 'Lean Manufacturing'),
-- User 4e6b349d
('4e6b349d', 'Financial Analysis'),
('4e6b349d', 'Budgeting'),
('4e6b349d', 'Financial Modeling'),
-- User 4e6b372f
('4e6b372f', 'Regulatory Compliance'),
('4e6b372f', 'Policy Analysis'),
('4e6b372f', 'Government Relations'),
-- User 4e6b396b
('4e6b396b', 'Contract Law'),
('4e6b396b', 'Legal Writing'),
('4e6b396b', 'Litigation Management'),
-- User 4e6b3be6
('4e6b3be6', 'Public Relations'),
('4e6b3be6', 'Media Relations'),
('4e6b3be6', 'Stakeholder Engagement'),
-- User 4e6b3f60
('4e6b3f60', 'Land Acquisition'),
('4e6b3f60', 'Environmental Impact Assessment'),
('4e6b3f60', 'Natural Resource Management'),
-- User 4e6b41a8
('4e6b41a8', 'Sustainable Design'),
('4e6b41a8', 'Environmental Planning'),
('4e6b41a8', 'Carbon Footprint Analysis'),
-- User 4e6b43db
('4e6b43db', 'Renewable Energy Technologies'),
('4e6b43db', 'Solar Energy Systems'),
('4e6b43db', 'Wind Power'),
-- User 4e6b46aa
('4e6b46aa', 'Human Resource Management'),
('4e6b46aa', 'Recruitment'),
('4e6b46aa', 'Employee Relations'),
-- User 4e6b4998
('4e6b4998', 'Database Management'),
('4e6b4998', 'Programming'),
('4e6b4998', 'Network Security'),
-- User 4e6b4c84
('4e6b4c84', 'Security Analysis'),
('4e6b4c84', 'Incident Response'),
('4e6b4c84', 'Cryptography'),
-- User 4e6b4f08
('4e6b4f08', 'Occupational Health and Safety'),
('4e6b4f08', 'Risk Assessment'),
('4e6b4f08', 'Emergency Response Planning'),
-- User 4e6b5530
('4e6b5530', 'Statistical Analysis'),
('4e6b5530', 'Experimental Design'),
('4e6b5530', 'Data Visualization'),
-- User 4e6b57e6
('4e6b57e6', 'Market Research'),
('4e6b57e6', 'Sales Strategies'),
('4e6b57e6', 'Customer Relationship Management'),
-- User 4e6b5a14
('4e6b5a14', 'Customer Support'),
('4e6b5a14', 'Complaint Resolution'),
('4e6b5a14', 'Product Knowledge'),
-- User 4e6b5ce4
('4e6b5ce4', 'Community Outreach'),
('4e6b5ce4', 'Event Planning'),
('4e6b5ce4', 'Public Speaking'),
-- User 4e6b5fbf
('4e6b5fbf', 'Social Media Management'),
('4e6b5fbf', 'Community Engagement'),
('4e6b5fbf', 'Crisis Communication'),
-- User 4e6b6242
('4e6b6242', 'Project Management'),
('4e6b6242', 'Team Leadership'),
('4e6b6242', 'Communication'),
('4e6b6242', 'Problem Solving'),
-- User 4e6b6491
('4e6b6491', 'Civil Engineering'),
('4e6b6491', 'Structural Analysis'),
('4e6b6491', 'AutoCAD'),
-- User 4e6b673b
('4e6b673b', 'Supply Chain Management'),
('4e6b673b', 'Vendor Negotiation'),
('4e6b673b', 'Inventory Management'),
-- User 4e6b696c
('4e6b696c', 'Construction Management'),
('4e6b696c', 'Safety Compliance'),
('4e6b696c', 'Project Scheduling'),
-- User 4e6b6bb6
('4e6b6bb6', 'Operations Management'),
('4e6b6bb6', 'Logistics'),
('4e6b6bb6', 'Lean Manufacturing'),
-- User 4e6b6de7
('4e6b6de7', 'Financial Analysis'),
('4e6b6de7', 'Budgeting'),
('4e6b6de7', 'Financial Modeling'),
-- User 4e6b7011
('4e6b7011', 'Regulatory Compliance'),
('4e6b7011', 'Policy Analysis'),
('4e6b7011', 'Government Relations'),
-- User 4e6b7243
('4e6b7243', 'Contract Law'),
('4e6b7243', 'Legal Writing'),
('4e6b7243', 'Litigation Management'),
-- User 4e6b74f1
('4e6b74f1', 'Public Relations'),
('4e6b74f1', 'Media Relations'),
('4e6b74f1', 'Stakeholder Engagement');

-- Project_T (10 Entries)
INSERT INTO Project_T (project_id, name, description, location, start_date, end_date, status, energy_rate_kwh, produced_energy_kwh, energy_sold_kwh, total_cost, org_restricted, m_p_user_id, creation_date) 
VALUES 
-- Projects starting in the future
('917fb980', 'Wind Turbine Installation', 'Installation of wind turbines for clean energy production.', 'Texas, USA', '2025-05-01', NULL, 'Planning', 0.08, 80000, 64000, 4500000, false, '4e6b2035', '2023-10-15'),
('917fbd99', 'Hydroelectric Power Plant Construction', 'Constructing a hydroelectric power plant on a river.', 'Oregon, USA', '2025-06-01', NULL, 'Pending Approval', 0.12, 120000, 96000, 7000000, true, '4e6b26a9', '2023-11-15'),

-- Projects starting in the past
('917fa1f1', 'Solar Farm Project', 'Developing a solar farm to generate renewable energy.', 'California, USA', '2024-04-01', '2025-04-01', 'In Progress', 0.10, 100000, 80000, 5000000, false, '4e6b1d60', '2023-03-15'),
('917fc0cf', 'Energy Efficiency Retrofitting', 'Retrofitting existing buildings with energy-efficient systems.', 'New York, USA', '2024-07-01', '2025-07-01', 'On Hold', 0.05, 50000, 40000, 3000000, false, '4e6b0f17', '2023-06-15'),
('917fc3c9', 'Biomass Power Generation Project', 'Utilizing biomass to generate electricity.', 'Georgia, USA', '2024-08-01', '2025-08-01', 'In Progress', 0.06, 60000, 48000, 3500000, false, '4e6b1152', '2023-07-15'),
('917fc6eb', 'Solar Panel Installation for Schools', 'Installing solar panels on school rooftops.', 'Florida, USA', '2024-09-01', '2025-09-01', 'Planning', 0.09, 90000, 72000, 5500000, false, '4e6b138f', '2023-08-15'),
('917fcdcf', 'Renewable Energy Research and Development', 'Researching new technologies for renewable energy.', 'Massachusetts, USA', '2024-10-01', '2025-10-01', 'In Progress', 0.11, 110000, 88000, 6500000, false, '4e6b15c6', '2023-09-15'),
('917fd0da', 'Solar Water Heating Project', 'Installing solar water heating systems in residential areas.', 'Arizona, USA', '2024-11-01', '2025-11-01', 'Planning', 0.07, 70000, 56000, 4000000, false, '4e6b18e4', '2023-10-15'),
('917fd39b', 'Geothermal Energy Exploration', 'Exploring geothermal energy potential in volcanic regions.', 'Hawaii, USA', '2024-12-01', '2025-12-01', 'Pending Approval', 0.13, 130000, 104000, 7500000, true, '4e6b1b2e', '2023-11-15'),
('917fd5d3', 'Community Solar Project', 'Establishing a community solar project for local residents.', 'Washington, USA', '2025-01-01', NULL, 'In Progress', 0.08, 80000, 64000, 5000000, false, '4e6b2397', '2023-12-15');

-- Investment_Proposal_T (10 Entries)
INSERT INTO Investment_Proposal_T (i_user_id, project_id, investment_amount, proposal_date, proposal_status) 
VALUES 
('4e6ae6d3', '917fb980', 50000, '2023-10-16', 'Pending'),
('4e6aece9', '917fa1f1', 75000, '2023-10-17', 'Submitted'),
('4e6af275', '917fd5d3', 100000, '2023-10-18', 'Reviewed'),
('4e6af650', '917fc6eb', 60000, '2023-10-19', 'Pending'),
('4e6afa6a', '917fc0cf', 90000, '2023-10-20', 'Approved'),
('4e6afef6', '917fd0da', 80000, '2023-10-21', 'Submitted'),
('4e6b02b3', '917fcdcf', 120000, '2023-10-22', 'Reviewed'),
('4e6b075f', '917fc3c9', 70000, '2023-10-23', 'Approved'),
('4e6b09b3', '917fb980', 95000, '2023-10-24', 'Pending'),
('4e6b0bf3', '917fa1f1', 85000, '2023-10-25', 'Submitted');

-- Investor_Invest_Project_T (10 Entries)
INSERT INTO Investor_Invest_Project_T (i_user_id, project_id, investment_amount, investment_date)
VALUES
('4e6afa6a', '917fc0cf', 90000, '2023-10-22'),
('4e6b075f', '917fc3c9', 70000, '2023-10-23');

-- Collaboration_T (30 Entries)
INSERT INTO Collaboration_T (p_user_id, project_id, start_date, end_date, role) VALUES
('4e6b5a14', '917fb980', '2023-10-17', '2023-12-31', 'Marketing and Sales Assistant'),
('4e6b5ce4', '917fb980', '2023-11-01', '2024-02-15', 'Customer Service Representative'),
('4e6b5fbf', '917fb980', '2023-11-18', '2023-11-30', 'Community Engagement Coordinator'),
('4e6b6242', '917fb980', '2023-12-01', '2024-01-31', 'Development Lead'),
('4e6b6491', '917fa1f1', '2023-10-21', '2023-12-15', 'Engineering Site Supervisor'),
('4e6b673b', '917fa1f1', '2023-11-01', '2024-01-31', 'Procurement Manager'),
('4e6b696c', '917fa1f1', '2023-11-16', '2023-11-30', 'Construction Manager'),
('4e6b6bb6', '917fa1f1', '2023-12-01', '2024-03-15', 'Operations Manager'),
('4e6b6de7', '917fa1f1', '2023-12-16', '2023-12-31', 'Financial Analyst'),
('4e6b7011', '917fc6eb', '2023-10-26', '2024-02-28', 'Regulatory Affairs Specialist'),
('4e6b7243', '917fc6eb', '2023-11-01', '2024-01-15', 'Legal Counsel'),
('4e6b74f1', '917fd0da', '2023-10-01', '2024-02-15', 'Public Affairs Officer'),
('4e6b28ee', '917fd0da', '2023-10-16', '2023-11-30', 'Development Engineer'),
('4e6b2b4b', '917fd0da', '2023-11-01', '2024-03-15', 'Engineering Specialist'),
('4e6b2d84', '917fd0da', '2023-12-01', '2024-01-31', 'Procurement Coordinator'),
('4e6b2fbb', '917fd0da', '2023-12-16', '2023-12-31', 'Construction Inspector'),
('4e6b3260', '917fd39b', '2023-10-01', '2024-02-28', 'Operations Coordinator'),
('4e6b349d', '917fd39b', '2023-11-01', '2024-01-15', 'Finance Manager'),
('4e6b372f', '917fd39b', '2023-12-01', '2024-03-15', 'Regulatory Affairs Manager'),
('4e6b396b', '917fd39b', '2023-12-16', '2023-12-31', 'Legal Consultant'),
('4e6b3be6', '917fc3c9', '2023-10-01', '2024-02-28', 'Public Affairs Specialist'),
('4e6b3f60', '917fc3c9', '2023-11-01', '2024-01-15', 'Land Management Specialist'),
('4e6b41a8', '917fc3c9', '2023-12-01', '2024-03-15', 'Environmental Specialist'),
('4e6b43db', '917fc3c9', '2023-12-16', '2023-12-31', 'Sustainability Analyst'),
('4e6b46aa', '917fd5d3', '2023-10-01', '2024-02-28', 'HR Coordinator'),
('4e6b4998', '917fd5d3', '2023-11-01', '2024-01-15', 'IT Specialist'),
('4e6b4c84', '917fd5d3', '2023-12-01', '2024-03-15', 'Security Analyst'),
('4e6b4f08', '917fd5d3', '2023-12-16', '2023-12-31', 'Health and Safety Officer'),
('4e6b5530', '917fc0cf', '2023-10-01', '2024-02-28', 'QA/QC Analyst'),
('4e6b57e6', '917fc0cf', '2023-11-01', '2024-01-15', 'R&D Engineer'),
('4e6b5a14', '917fc0cf', '2023-12-01', '2024-03-15', 'Sales Representative');

-- Collaboration_Task_T (60 Entries)
INSERT INTO Collaboration_Task_T (p_user_id, project_id, task_name, status, assigned_date, start_date, delivery_date, expected_hour, hour_taken, expected_day)
VALUES
('4e6b5a14', '917fb980', 'Marketing Strategy Development', 'In Progress', '2023-10-17', '2023-10-18', '2023-11-15', 40, 0, 20),
('4e6b5a14', '917fb980', 'Sales Campaign Planning', 'Pending', '2023-10-17', '2023-10-19', '2023-11-25', 30, 0, 15),
('4e6b5ce4', '917fb980', 'Customer Support Training', 'In Progress', '2023-11-01', '2023-11-02', '2023-11-30', 60, 0, 20),
('4e6b5ce4', '917fb980', 'Complaint Resolution Guidelines Development', 'Pending', '2023-11-01', '2023-11-03', '2023-12-05', 50, 0, 25),
('4e6b5fbf', '917fb980', 'Community Outreach Plan Creation', 'In Progress', '2023-11-18', '2023-11-19', '2023-12-10', 35, 0, 20),
('4e6b5fbf', '917fb980', 'Stakeholder Engagement Strategy Development', 'Pending', '2023-11-18', '2023-11-20', '2023-12-15', 25, 0, 15),
('4e6b6242', '917fb980', 'Development Plan Analysis', 'In Progress', '2023-12-01', '2023-12-02', '2023-12-20', 45, 0, 18),
('4e6b6242', '917fb980', 'Prototype Development', 'Pending', '2023-12-01', '2023-12-03', '2024-01-05', 55, 0, 30),
('4e6b6491', '917fa1f1', 'Site Inspection', 'In Progress', '2023-10-21', '2023-10-22', '2023-11-10', 40, 0, 15),
('4e6b6491', '917fa1f1', 'Engineering Design Review', 'Pending', '2023-10-21', '2023-10-23', '2023-12-01', 60, 0, 30),
('4e6b673b', '917fa1f1', 'Vendor Selection', 'In Progress', '2023-11-01', '2023-11-02', '2023-11-25', 30, 0, 15),
('4e6b673b', '917fa1f1', 'Procurement Negotiations', 'Pending', '2023-11-01', '2023-11-03', '2023-12-05', 50, 0, 25),
('4e6b696c', '917fa1f1', 'Construction Planning', 'In Progress', '2023-11-16', '2023-11-17', '2023-12-10', 35, 0, 20),
('4e6b696c', '917fa1f1', 'Safety Compliance Training', 'Pending', '2023-11-16', '2023-11-18', '2023-12-15', 25, 0, 15),
('4e6b6bb6', '917fa1f1', 'Operations Strategy Development', 'In Progress', '2023-12-01', '2023-12-02', '2023-12-20', 45, 0, 18),
('4e6b6bb6', '917fa1f1', 'Logistics Optimization', 'Pending', '2023-12-01', '2023-12-03', '2024-01-05', 55, 0, 30),
('4e6b6de7', '917fa1f1', 'Financial Analysis', 'In Progress', '2023-12-16', '2023-12-17', '2024-01-10', 40, 0, 15),
('4e6b6de7', '917fa1f1', 'Budget Planning', 'Pending', '2023-12-16', '2023-12-18', '2024-02-01', 60, 0, 45),
('4e6b7011', '917fc6eb', 'Regulatory Compliance Assessment', 'In Progress', '2023-10-26', '2023-10-27', '2023-11-15', 30, 0, 15),
('4e6b7011', '917fc6eb', 'Policy Review', 'Pending', '2023-10-26', '2023-10-28', '2023-12-05', 50, 0, 25),
('4e6b7243', '917fc6eb', 'Legal Document Drafting', 'In Progress', '2023-11-01', '2023-11-02', '2023-11-30', 35, 0, 20),
('4e6b7243', '917fc6eb', 'Contract Negotiations', 'Pending', '2023-11-01', '2023-11-03', '2023-12-15', 25, 0, 15),
('4e6b74f1', '917fd0da', 'PR Strategy Planning', 'In Progress', '2023-10-01', '2023-10-02', '2023-10-20', 45, 0, 18),
('4e6b74f1', '917fd0da', 'Media Relations Outreach', 'Pending', '2023-10-01', '2023-10-03', '2023-11-05', 55, 0, 30),
('4e6b28ee', '917fd0da', 'Development Plan Analysis', 'In Progress', '2023-10-16', '2023-10-17', '2023-11-15', 40, 0, 20),
('4e6b28ee', '917fd0da', 'Prototype Development', 'Pending', '2023-10-16', '2023-10-18', '2023-12-01', 60, 0, 30),
('4e6b2b4b', '917fd0da', 'Engineering Design Review', 'In Progress', '2023-11-01', '2023-11-02', '2023-11-25', 30, 0, 15),
('4e6b2b4b', '917fd0da', 'Technical Feasibility Analysis', 'Pending', '2023-11-01', '2023-11-03', '2023-12-05', 50, 0, 25),
('4e6b2d84', '917fd0da', 'Vendor Selection', 'In Progress', '2023-12-01', '2023-12-02', '2023-12-20', 45, 0, 18),
('4e6b2d84', '917fd0da', 'Procurement Negotiations', 'Pending', '2023-12-01', '2023-12-03', '2024-01-05', 55, 0, 30),
('4e6b2fbb', '917fd0da', 'Construction Planning', 'In Progress', '2023-12-16', '2023-12-17', '2024-01-10', 40, 0, 15),
('4e6b2fbb', '917fd0da', 'Safety Compliance Training', 'Pending', '2023-12-16', '2023-12-18', '2024-02-01', 60, 0, 45),
('4e6b3260', '917fd39b', 'Operations Strategy Development', 'In Progress', '2023-10-01', '2023-10-02', '2023-10-20', 45, 0, 18),
('4e6b3260', '917fd39b', 'Logistics Optimization', 'Pending', '2023-10-01', '2023-10-03', '2023-11-05', 55, 0, 30),
('4e6b349d', '917fd39b', 'Financial Analysis', 'In Progress', '2023-11-01', '2023-11-02', '2023-11-25', 30, 0, 15),
('4e6b349d', '917fd39b', 'Budget Planning', 'Pending', '2023-11-01', '2023-11-03', '2023-12-05', 50, 0, 25),
('4e6b372f', '917fd39b', 'Regulatory Compliance Assessment', 'In Progress', '2023-12-01', '2023-12-02', '2023-12-20', 45, 0, 18),
('4e6b372f', '917fd39b', 'Policy Review', 'Pending', '2023-12-01', '2023-12-03', '2024-01-05', 55, 0, 30),
('4e6b396b', '917fd39b', 'Legal Document Drafting', 'In Progress', '2023-12-16', '2023-12-17', '2024-01-10', 40, 0, 15),
('4e6b396b', '917fd39b', 'Contract Negotiations', 'Pending', '2023-12-16', '2023-12-18', '2024-02-01', 60, 0, 45),
('4e6b3be6', '917fc3c9', 'PR Strategy Planning', 'In Progress', '2023-10-01', '2023-10-02', '2023-10-20', 45, 0, 18),
('4e6b3be6', '917fc3c9', 'Media Relations Outreach', 'Pending', '2023-10-01', '2023-10-03', '2023-11-05', 55, 0, 30),
('4e6b3f60', '917fc3c9', 'Development Plan Analysis', 'In Progress', '2023-11-01', '2023-11-02', '2023-11-25', 30, 0, 15),
('4e6b3f60', '917fc3c9', 'Prototype Development', 'Pending', '2023-11-01', '2023-11-03', '2023-12-05', 50, 0, 25),
('4e6b41a8', '917fc3c9', 'Engineering Design Review', 'In Progress', '2023-12-01', '2023-12-02', '2023-12-20', 45, 0, 18),
('4e6b41a8', '917fc3c9', 'Technical Feasibility Analysis', 'Pending', '2023-12-01', '2023-12-03', '2024-01-05', 55, 0, 30),
('4e6b43db', '917fc3c9', 'Vendor Selection', 'In Progress', '2023-12-16', '2023-12-17', '2024-01-10', 40, 0, 15),
('4e6b43db', '917fc3c9', 'Procurement Negotiations', 'Pending', '2023-12-16', '2023-12-18', '2024-02-01', 60, 0, 45),
('4e6b46aa', '917fc3c9', 'Construction Planning', 'In Progress', '2024-01-01', '2024-01-02', '2024-01-20', 45, 0, 18),
('4e6b46aa', '917fc3c9', 'Safety Compliance Training', 'Pending', '2024-01-01', '2024-01-03', '2024-02-05', 55, 0, 30),
('4e6b4998', '917fd5d3', 'Operations Strategy Development', 'In Progress', '2023-10-01', '2023-10-02', '2023-10-20', 45, 0, 18),
('4e6b4998', '917fd5d3', 'Logistics Optimization', 'Pending', '2023-10-01', '2023-10-03', '2023-11-05', 55, 0, 30),
('4e6b4c84', '917fd5d3', 'Financial Analysis', 'In Progress', '2023-11-01', '2023-11-02', '2023-11-25', 30, 0, 15),
('4e6b4c84', '917fd5d3', 'Budget Planning', 'Pending', '2023-11-01', '2023-11-03', '2023-12-05', 50, 0, 25),
('4e6b4f08', '917fd5d3', 'Regulatory Compliance Assessment', 'In Progress', '2023-12-01', '2023-12-02', '2023-12-20', 45, 0, 18),
('4e6b4f08', '917fd5d3', 'Policy Review', 'Pending', '2023-12-01', '2023-12-03', '2024-01-05', 55, 0, 30),
('4e6b5530', '917fd5d3', 'Legal Document Drafting', 'In Progress', '2024-01-01', '2024-01-02', '2024-01-30', 40, 0, 20),
('4e6b5530', '917fd5d3', 'Contract Negotiations', 'Pending', '2024-01-01', '2024-01-03', '2024-02-15', 60, 0, 45);

-- Show data
SELECT * FROM Organization_T;
SELECT * FROM Organization_Contact_T;
SELECT * FROM User_T;
SELECT * FROM Investor_T;
SELECT * FROM Project_Associate_T;
SELECT * FROM Project_Manager_T;
SELECT * FROM Project_Contributor_T;
SELECT * FROM Project_Contributor_Skill_T;
SELECT * FROM Project_T;
SELECT * FROM Investment_Proposal_T;
SELECT * FROM Investor_Invest_Project_T;
SELECT * FROM Collaboration_T;
SELECT * FROM Collaboration_Task_T;