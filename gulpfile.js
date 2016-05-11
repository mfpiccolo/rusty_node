const gulp = require('gulp'),
      babel = require('gulp-babel'),
      watch = require('gulp-watch'),
      batch = require('gulp-batch'),
      concat = require('gulp-concat'),
      rename = require('gulp-rename'),
      uglify = require('gulp-uglify'),
      sourcemaps = require('gulp-sourcemaps');

gulp.task('js-fef', function(){
  return gulp.src([
      'lib/diesel_node/*.js',
      'lib/models/*.js',
      'lib/tables/*.js',
      'lib/index.js'])
    .pipe(babel({presets: ['es2015']}))
    // .pipe(sourcemaps.init())
    .pipe(concat('index.js'))
    .pipe(gulp.dest('./dist'))
    // .pipe(rename('uglify.js'))
    // .pipe(uglify())
    // .pipe(sourcemaps.write('./'))
    .pipe(gulp.dest('dist'));
});

gulp.task('default', ['js-fef'], function(){});

gulp.task('watch', function () {
  watch('lib/*.js', batch(function (events, done) {
      gulp.start('default', done);
  }));
});
