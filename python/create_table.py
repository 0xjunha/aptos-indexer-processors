from sqlalchemy import BigInteger, Column, create_engine, DateTime, String
from sqlalchemy.orm import declarative_base

import argparse

from config import Config

parser = argparse.ArgumentParser()
parser.add_argument("-c", "--config", help="Path to config file", required=True)
args = parser.parse_args()

config = Config.from_yaml_file(args.config)

Base = declarative_base()


class Event(Base):
    __tablename__ = "events"

    sequence_number = Column(BigInteger, primary_key=True)
    creation_number = Column(BigInteger, primary_key=True)
    account_address = Column(String, primary_key=True)
    transaction_version = Column(BigInteger)
    transaction_block_height = Column(BigInteger)
    type = Column(String)
    data = Column(String)
    inserted_at = Column(DateTime)
    event_index = Column(BigInteger)


engine = create_engine(config.db_connection_uri)
Base.metadata.create_all(engine)