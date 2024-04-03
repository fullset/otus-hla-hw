import psycopg2
import csv
import uuid


# пытаемся подключиться к базе данных
conn = psycopg2.connect(dbname='social_net', user='postgres', password='adminpgpwd4otushw', host='127.0.0.1', port=6432)

with open("../../research/people.csv") as fp:
    reader = csv.reader(fp, delimiter=",", quotechar='"')
    counter = 1
    with conn.cursor() as cur:
        for row in reader:
            nextuuid = uuid.uuid4()
            print(counter, row[0], row[1], row[2], row[3], str(nextuuid))
            cur.execute(
                'INSERT INTO social_net.users (second_name, first_name, birthdate, city, user_id, password_hash) VALUES (%s, %s, current_date - interval %s year, %s, %s, %s)',
                (row[0], row[1], row[2], row[3], str(nextuuid), 'research'))
            counter=counter+1
            conn.commit()
    conn.close()

