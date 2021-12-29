# frozen_string_literal: true

require 'colorize'
require 'sqlite3'
require 'yaml'
require 'faraday'
require 'faraday_middleware/response/follow_redirects'
require 'fileutils'

require_relative './metadata'
require_relative './db_create'
require_relative './images_downloader'