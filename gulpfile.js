const gulp = require('gulp');
const babel = require('gulp-babel');
const watch = require('gulp-watch');
const batch = require('gulp-batch');

gulp.task('default', () => {
  return gulp.src('lib/index.js')
    .pipe(babel({
      presets: ['es2015']
    }))
    .pipe(gulp.dest('dist'));
});

gulp.task('watch', function () {
    watch('lib/*.js', batch(function (events, done) {
        gulp.start('default', done);
    }));
});
